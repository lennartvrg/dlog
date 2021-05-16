use directories::ProjectDirs;
use std::cmp::min;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::timeout;

use crate::ingest::HttpIngestor;
use crate::models::{Log, Priority};
use crate::worker::Signal;

pub enum BacklogSignal {
    Entries(Vec<Log>),
    Flush,
    Exit,
}

pub struct Backlog {
    signal_receiver: flume::Receiver<BacklogSignal>,
    log_sender: flume::Sender<Signal>,
    ingest: Arc<HttpIngestor>,
    dirs: ProjectDirs,
    exit: bool,
    queue: Vec<Log>,
    backoff_multiplier: u32,
    pub is_empty: Arc<AtomicBool>,
    pub signal_sender: flume::Sender<BacklogSignal>,
    flush_sender: flume::Sender<()>,
    pub flush_receiver: flume::Receiver<()>,
}

const BACKLOG_CHUNK_SIZE: usize = 1_000;
const BACKLOG_CHECK_INTERVAL: Duration = Duration::from_secs(10);
const BACKLOG_MAX_CHECK_INTERVAL: Duration = Duration::from_secs(120);
const BACKLOG_MIN_LOOP_INTERVAL: Duration = Duration::from_millis(100);

impl Backlog {
    pub fn new(ingest: Arc<HttpIngestor>, log_sender: flume::Sender<Signal>) -> Self {
        let dirs = ProjectDirs::from("cloud.dlog", "", "dlog").unwrap();
        let (signal_sender, signal_receiver) = flume::unbounded();
        let (flush_sender, flush_receiver) = flume::unbounded();

        Self {
            dirs,
            ingest,
            log_sender,
            signal_receiver,
            exit: false,
            backoff_multiplier: 1,
            queue: Vec::with_capacity(BACKLOG_CHUNK_SIZE),
            is_empty: Arc::new(AtomicBool::new(true)),
            signal_sender,
            flush_sender,
            flush_receiver,
        }
    }

    pub async fn start(&mut self) {
        let mut last_check = Instant::now();
        self.load_from_disk().await;

        if let Err(err) = self.flush_sender.send_async(()).await {
            eprintln!("[dlog::worker] Failed to receive ready signal: {}", err);
        }

        while !self.exit {
            if let Ok(val) = timeout(BACKLOG_MIN_LOOP_INTERVAL, self.signal_receiver.recv_async()).await {
                self.receive(val).await;
            }

            if last_check.elapsed() >= self.backoff() {
                last_check = Instant::now();
                self.retry().await;
            }
        }

        self.flush_to_disk().await;
    }

    async fn receive(&mut self, signal: Result<BacklogSignal, flume::RecvError>) {
        self.is_empty.store(false, Ordering::Relaxed);
        match signal {
            Ok(BacklogSignal::Entries(mut logs)) => self.queue.append(&mut logs),
            Ok(BacklogSignal::Flush) => {
                while let Ok(signal) = self.signal_receiver.try_recv() {
                    if let BacklogSignal::Entries(mut logs) = signal {
                        self.queue.append(&mut logs);
                    }
                }

                self.retry().await;
                if let Err(err) = self.flush_sender.send_async(()).await {
                    eprintln!("[dlog::backlog] Cannot send flush signal: {}", err);
                }
            }
            _ => self.exit = true,
        }
    }

    async fn retry(&mut self) {
        if !self.queue.is_empty() && self.ingest.check().await {
            self.load_from_disk().await;
            self.is_empty.store(true, Ordering::Relaxed);
            self.send_log(format!("[dlog] Retrying ingest for {} logs", self.queue.len()))
                .await;
            while !self.queue.is_empty() {
                let mut logs = self
                    .queue
                    .drain(..min(self.queue.len(), BACKLOG_CHUNK_SIZE))
                    .collect::<Vec<Log>>();

                if let Err(err) = self.ingest.log_async(&logs).await {
                    self.queue.append(&mut logs);
                    self.queue.push(err);
                    self.backoff_multiplier += 1;
                    self.send_log(format!("[dlog] Will retry in {} seconds", self.backoff().as_secs()))
                        .await;
                    return;
                }
            }
            self.backoff_multiplier = 1;
        } else if !self.queue.is_empty() {
            self.flush_to_disk().await;
        }
    }

    fn backoff(&self) -> Duration {
        min(
            BACKLOG_CHECK_INTERVAL * self.backoff_multiplier,
            BACKLOG_MAX_CHECK_INTERVAL,
        )
    }

    async fn flush_to_disk(&mut self) {
        if !self.queue.is_empty() {
            if let Some(mut file) = Self::get_file(&self.dirs, true) {
                for log in self.queue.drain(..self.queue.len()) {
                    if let Ok(ctn) = serde_json::to_string(&log) {
                        if let Err(err) = writeln!(file, "{}", ctn) {
                            eprintln!("[dlog::backlog] Cannot write log to cache: {}", err);
                        }
                    }
                }
            }
        }
    }

    async fn load_from_disk(&mut self) {
        if let Some(file) = Self::get_file(&self.dirs, false) {
            let reader = BufReader::new(file);
            let mut logs = Vec::new();
            for line in reader.lines() {
                if let Ok(line) = line {
                    if let Ok(log) = serde_json::from_str::<Log>(&line) {
                        logs.push(log);
                    }
                }
            }
            logs.append(&mut self.queue);
            self.queue = logs;
            Self::remove_file(&self.dirs);
        }
    }

    async fn send_log(&self, message: impl Into<String>) {
        let _ = self
            .log_sender
            .send_async(Signal::Log(Log::new(Priority::Trace, message)))
            .await;
    }

    fn get_file(dirs: &ProjectDirs, create: bool) -> Option<std::fs::File> {
        match Self::get_path(dirs) {
            None => None,
            Some(path) => OpenOptions::new()
                .read(true)
                .append(true)
                .create(create)
                .open(path)
                .ok(),
        }
    }

    fn remove_file(dirs: &ProjectDirs) {
        if let Some(path) = Self::get_path(dirs) {
            let _ = std::fs::remove_file(path);
        }
    }

    fn get_path(dirs: &ProjectDirs) -> Option<PathBuf> {
        let path = dirs.config_dir();
        if !path.exists() && std::fs::create_dir_all(&path).is_err() {
            eprintln!("[dlog::backlog] Cannot cache logs to {:?}", path);
            return None;
        }
        Some(path.join("backlog.dat"))
    }
}
