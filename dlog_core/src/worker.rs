use futures::{future::FutureExt, select};
use std::sync::Arc;
use flume::RecvError;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::ingestor::HttpIngestor;
use crate::models::Log;

const DEFAULT_QUEUE_LENGTH: usize = 1_000;
const MIN_FLUSH_INTERVAL: std::time::Duration = std::time::Duration::from_secs(1);

pub enum Signal {
    HasValidApiKey(bool),
    Log(Log),
    Flush,
    None,
}

pub struct Worker {
    terminator: Arc<AtomicBool>,
    signal_receiver: flume::Receiver<Signal>,
    ingest: HttpIngestor,
    queue: Vec<Log>,
}

impl Worker {
    pub fn new(api_key: String, terminator: Arc<AtomicBool>, signal_receiver: flume::Receiver<Signal>) -> Self {
        Self {
            signal_receiver,
            terminator,
            ingest: HttpIngestor::new(api_key),
            queue: Vec::with_capacity(DEFAULT_QUEUE_LENGTH)
        }
    }

    async fn start(&mut self) {
        while !self.terminator.load(Ordering::Relaxed) {
            select! {
                signal = self.signal_receiver.recv_async() => self.receive(signal),
                () = tokio::time::sleep(MIN_FLUSH_INTERVAL).fuse() => self.flush(),
            }
        }
    }

    fn receive(&mut self, res: Result<Signal, RecvError>) {
        match res {
            Err(_) => self.terminator.store(true, Ordering::Relaxed),
            Ok(Signal::Log(log)) => self.queue.push(log),
            Ok(Signal::Flush) => self.flush(),
            _ => (),
        };
    }

    fn flush(&mut self) {
        if self.queue.len() > 0 {
            self.ingest.log()
        }
    }
}