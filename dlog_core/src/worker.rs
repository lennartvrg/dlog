use flume::RecvError;
use futures::{future::FutureExt, select};

use crate::ingestor::HttpIngestor;
use crate::models::Log;
use crate::transforms::{Transform, Transforms};

const DEFAULT_QUEUE_LENGTH: usize = 1_000;
const MIN_FLUSH_INTERVAL: std::time::Duration = std::time::Duration::from_secs(1);

pub enum Signal {
    Log(Log),
    Flush,
    Exit,
}

pub struct Worker {
    exit: bool,
    queue: Vec<Log>,
    ingest: HttpIngestor,
    transforms: Transforms,
    signal_sender: flume::Sender<Signal>,
    signal_receiver: flume::Receiver<Signal>,
    flush_sender: flume::Sender<()>,
}

impl Worker {
    pub fn new(
        api_key: String,
        transforms: Transforms,
    ) -> Result<(Self, flume::Sender<Signal>, flume::Receiver<()>), String> {
        let ingest = HttpIngestor::new(api_key);

        let (signal_sender, signal_receiver) = flume::bounded(DEFAULT_QUEUE_LENGTH);
        let (flush_sender, flush_receiver) = flume::unbounded();

        let instance = Self {
            exit: false,
            queue: Vec::with_capacity(DEFAULT_QUEUE_LENGTH),
            ingest,
            transforms,
            signal_sender: signal_sender.clone(),
            signal_receiver,
            flush_sender,
        };

        Ok((instance, signal_sender, flush_receiver))
    }

    pub async fn start(&mut self) {
        while !self.exit {
            select! {
                signal = self.signal_receiver.recv_async() => self.receive(signal).await,
                () = tokio::time::sleep(MIN_FLUSH_INTERVAL).fuse() => self.flush().await,
            }
        }
        self.flush().await;
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
        if self.queue.len() > 0 {
            let logs = self.queue.drain(..).collect::<Vec<Log>>();
            if let Err(log) = self.ingest.log_async(&logs).await {
                if let Err(err) = self.signal_sender.send_async(Signal::Log(log)).await {
                    println!("[dlog] Failed to send signal: {}", err);
                }
            }
        }
    }
}
