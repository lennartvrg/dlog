use flume::RecvError;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::timeout;

use crate::backlog::{Backlog, BacklogSignal};
use crate::ingest::HttpIngestor;
use crate::models::Log;
use crate::transforms::{Transform, Transforms};

const DEFAULT_QUEUE_LENGTH: usize = 1_000;
const MIN_LOOP_INTERVAL: Duration = Duration::from_millis(50);
const MIN_FLUSH_INTERVAL: Duration = Duration::from_secs(1);

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
    flush_sender: flume::Sender<()>,
    backlog_sender: flume::Sender<BacklogSignal>,
    is_backlog_empty: Arc<AtomicBool>,
}

impl Worker {
    pub fn new(
        api_key: String,
        transforms: Transforms,
    ) -> Result<(Self, Backlog, flume::Sender<Signal>, flume::Receiver<()>), String> {
        let ingest = Arc::new(HttpIngestor::new(api_key));

        let (signal_sender, signal_receiver) = flume::bounded(DEFAULT_QUEUE_LENGTH);
        let (flush_sender, flush_receiver) = flume::unbounded();
        let (backlog, is_backlog_empty, backlog_sender) = Backlog::new(ingest.clone());

        let instance = Self {
            exit: false,
            queue: Vec::with_capacity(DEFAULT_QUEUE_LENGTH),
            ingest,
            transforms,
            signal_receiver,
            flush_sender,
            backlog_sender,
            is_backlog_empty,
        };

        Ok((instance, backlog, signal_sender, flush_receiver))
    }

    pub async fn has_valid_api_key(&self) -> bool {
        self.ingest.has_valid_api_key().await
    }

    pub async fn start(&mut self) {
        let mut last_check = Instant::now();
        while !self.exit {
            if let Ok(val) = timeout(MIN_LOOP_INTERVAL, self.signal_receiver.recv_async()).await {
                self.receive(val).await;
            }

            if last_check.elapsed() >= MIN_FLUSH_INTERVAL {
                last_check = Instant::now();
                self.flush().await;
            }
        }

        self.flush().await;
        if let Err(err) = self.backlog_sender.send_async(BacklogSignal::Exit).await {
            eprintln!("[dlog] Could not send exit signal to backlog: {}", err);
        };
    }

    async fn receive(&mut self, res: Result<Signal, RecvError>) {
        match res {
            Ok(Signal::Log(log)) => self.add(log).await,
            Ok(Signal::Flush) => {
                self.flush().await;
                if let Err(err) = self.flush_sender.send(()) {
                    println!("[dlog] Failed to send flush signal: {}", err);
                }
            }
            _ => self.exit = true,
        };
    }

    async fn add(&mut self, mut log: Log) {
        self.transforms.apply(&mut log);
        self.queue.push(log);
        if self.queue.len() >= DEFAULT_QUEUE_LENGTH {
            self.flush().await;
        }
    }

    async fn flush(&mut self) {
        if !self.queue.is_empty() {
            let mut logs = self.queue.drain(..).collect::<Vec<Log>>();
            if !self.is_backlog_empty.load(Ordering::Relaxed) {
                if let Err(err) = self.backlog_sender.send_async(BacklogSignal::Entries(logs)).await {
                    println!("[dlog] Failed to send signal: {}", err);
                }
            } else if let Err(log) = self.ingest.log_async(&logs).await {
                logs.push(log);
                if let Err(err) = self.backlog_sender.send_async(BacklogSignal::Entries(logs)).await {
                    println!("[dlog] Failed to send signal: {}", err);
                }
            }
        }
    }
}
