use flume::RecvError;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::backlog::{Backlog, BacklogSignal};
use crate::ingest::HttpIngestor;
use crate::models::Log;
use crate::transforms::{Transform, Transforms};
use std::cmp::min;

const FLUSH_CHUNK_SIZE: usize = 1_000;
const DEFAULT_QUEUE_LENGTH: usize = 100_000;
const MIN_FLUSH_INTERVAL: Duration = Duration::from_secs(1);
const MIN_LOOP_INTERVAL: Duration = Duration::from_millis(50);

pub enum Signal {
    Log(Log),
    Flush,
    Exit,
}

pub struct Worker {
    exit: bool,
    queue: Vec<Log>,
    ingest: Arc<HttpIngestor>,
    transforms: Transforms,
    signal_receiver: flume::Receiver<Signal>,
    pub signal_sender: flume::Sender<Signal>,
    flush_sender: flume::Sender<()>,
    pub flush_receiver: flume::Receiver<()>,
    backlog_sender: flume::Sender<BacklogSignal>,
    is_backlog_empty: Arc<AtomicBool>,
    backlog_flush_receiver: flume::Receiver<()>,
}

impl Worker {
    pub fn new(api_key: String, transforms: Transforms) -> Result<(Self, Backlog), String> {
        let ingest = Arc::new(HttpIngestor::new(api_key)?);

        let (signal_sender, signal_receiver) = flume::unbounded();
        let (flush_sender, flush_receiver) = flume::unbounded();
        let backlog = Backlog::new(ingest.clone(), signal_sender.clone());

        let instance = Self {
            exit: false,
            queue: Vec::with_capacity(DEFAULT_QUEUE_LENGTH),
            ingest,
            transforms,
            signal_receiver,
            signal_sender,
            flush_sender,
            flush_receiver,
            backlog_sender: backlog.signal_sender.clone(),
            is_backlog_empty: backlog.is_empty.clone(),
            backlog_flush_receiver: backlog.flush_receiver.clone(),
        };

        Ok((instance, backlog))
    }

    pub async fn has_valid_api_key(&self) -> bool {
        self.ingest.has_valid_api_key().await
    }

    pub async fn start(&mut self) {
        if let Err(err) = self.backlog_flush_receiver.recv_async().await {
            eprintln!("[dlog::worker] Failed to receive ready signal: {}", err);
        }

        if let Err(err) = self.flush_sender.send_async(()).await {
            eprintln!("[dlog::logger] Failed to send ready signal: {}", err);
        }

        let mut last_check = Instant::now();
        while !self.exit {
            while let Ok(val) = self.signal_receiver.try_recv() {
                self.receive(Ok(val)).await;
            }

            tokio::time::sleep(MIN_LOOP_INTERVAL).await;
            if last_check.elapsed() >= MIN_FLUSH_INTERVAL {
                last_check = Instant::now();
                self.flush().await;
            }
        }

        self.flush().await;
        if let Err(err) = self.backlog_sender.send_async(BacklogSignal::Exit).await {
            eprintln!("[dlog::worker] Could not send exit signal to backlog: {}", err);
        };
        if let Err(err) = self.flush_sender.send_async(()).await {
            eprintln!("[dlog::worker] Failed to respond to exit signal: {}", err);
        }
    }

    async fn receive(&mut self, res: Result<Signal, RecvError>) {
        match res {
            Ok(Signal::Log(log)) => self.add(log).await,
            Ok(Signal::Flush) => {
                self.flush().await;
                if let Err(err) = self.backlog_sender.send_async(BacklogSignal::Flush).await {
                    eprintln!("[dlog::worker] Failed to send flush signal to backlog: {}", err);
                }

                if let Err(err) = self.backlog_flush_receiver.recv_async().await {
                    eprintln!("[dlog::worker] Failed to receive flush signal from backlog: {}", err);
                }

                if let Err(err) = self.flush_sender.send_async(()).await {
                    eprintln!("[dlog::worker] Failed to respond to flush signal: {}", err);
                }
            }
            _ => self.exit = true,
        };
    }

    async fn add(&mut self, mut log: Log) {
        self.transforms.apply(&mut log);
        self.queue.push(log);
        if self.queue.len() >= FLUSH_CHUNK_SIZE {
            self.flush().await;
        }
    }

    async fn flush(&mut self) {
        if !self.queue.is_empty() {
            let logs = self.queue.drain(..min(self.queue.len(), FLUSH_CHUNK_SIZE)).collect::<Vec<Log>>();
            if !self.is_backlog_empty.load(Ordering::Relaxed) {
                if let Err(err) = self.backlog_sender.send_async(BacklogSignal::Entries(logs)).await {
                    eprintln!("[dlog::worker] Failed to send backlog signal: {}", err);
                }
            } else if let Err(log) = self.ingest.log_async(&logs).await {
                if let Err(err) = self.backlog_sender.send_async(BacklogSignal::Entries(logs)).await {
                    eprintln!("[dlog::worker] Failed to send backlog signal: {}", err);
                }

                if let Err(err) = self.signal_sender.send_async(Signal::Log(log)).await {
                    eprintln!("[dlog::worker] Failed to send signal: {}", err);
                }
            }
        }
    }
}
