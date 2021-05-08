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
use crate::models::Log;

pub enum BacklogSignal {
    Entries(Vec<Log>),
    Exit,
}

pub struct Backlog {
    signal_receiver: flume::Receiver<BacklogSignal>,
    ingest: Arc<HttpIngestor>,
    dirs: ProjectDirs,
    exit: bool,
    queue: Vec<Log>,
    is_empty: Arc<AtomicBool>,
}

const BACKLOG_CHUNK_SIZE: usize = 1_000;
const BACKLOG_CHECK_INTERVAL: Duration = Duration::from_secs(5);
const BACKLOG_MIN_LOOP_INTERVAL: Duration = Duration::from_millis(100);

impl Backlog {
    pub fn new(ingest: Arc<HttpIngestor>) -> (Self, Arc<AtomicBool>, flume::Sender<BacklogSignal>) {
        let dirs = ProjectDirs::from("cloud.dlog", "", "dlog").unwrap();
        let (signal_sender, signal_receiver) = flume::unbounded();
        let is_empty = Arc::new(AtomicBool::new(true));
        (
            Self {
                dirs,
                ingest,
                signal_receiver,
                exit: false,
                queue: Vec::with_capacity(BACKLOG_CHUNK_SIZE),
                is_empty: is_empty.clone(),
            },
            is_empty,
            signal_sender,
        )
    }

    pub async fn start(&mut self) {
        let mut last_check = Instant::now();
        self.load_from_disk();

        while !self.exit {
            if let Ok(val) = timeout(BACKLOG_MIN_LOOP_INTERVAL, self.signal_receiver.recv_async()).await {
                self.receive(val).await;
            }

            if last_check.elapsed() >= BACKLOG_CHECK_INTERVAL {
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
            _ => self.exit = true,
        }
    }

    async fn retry(&mut self) {
        if !self.queue.is_empty() && self.ingest.check().await {
            self.load_from_disk();
            self.is_empty.store(true, Ordering::Relaxed);
            while !self.queue.is_empty() {
                let mut logs = self
                    .queue
                    .drain(..min(self.queue.len(), BACKLOG_CHUNK_SIZE))
                    .collect::<Vec<Log>>();
                if let Err(err) = self.ingest.log_async(&logs).await {
                    self.queue.append(&mut logs);
                    self.queue.push(err);
                    return;
                }
            }
        } else if !self.queue.is_empty() {
            self.flush_to_disk().await;
        }
    }

    async fn flush_to_disk(&mut self) {
        if !self.queue.is_empty() {
            if let Some(mut file) = Self::get_file(&self.dirs, true) {
                for log in self.queue.drain(..self.queue.len()) {
                    if let Ok(ctn) = serde_json::to_string(&log) {
                        if let Err(err) = writeln!(file, "{}", ctn) {
                            eprintln!("[dlog] Cannot write log to cache: {}", err);
                        }
                    }
                }
            }
        }
    }

    fn load_from_disk(&mut self) {
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
            eprintln!("[dlog] Cannot cache logs to {:?}", path);
            return None;
        }
        Some(path.join("backlog.dat"))
    }
}
