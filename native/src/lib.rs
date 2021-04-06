use crate::ingestor::HttpIngestor;
use crate::models::{Log, Priority};

mod ingestor;
pub mod models;

const QUEUE_BUFFER: usize = 1_000;

const MIN_FLUSH_INTERVAL: std::time::Duration = std::time::Duration::from_secs(1);

#[derive(Debug)]
pub struct Logger {
    sender: flume::Sender<Log>,
}

impl Logger {
    pub fn new(api_key: String) -> Self {
        let (sender, receiver) = flume::bounded::<Log>(QUEUE_BUFFER);

        std::thread::spawn(move || {
            let client = HttpIngestor::new(api_key);
            let mut queue = Vec::<Log>::with_capacity(QUEUE_BUFFER);

            loop {
                let flush = match receiver.recv_timeout(MIN_FLUSH_INTERVAL) {
                    Err(flume::RecvTimeoutError::Disconnected) => break,
                    Err(flume::RecvTimeoutError::Timeout) => true,
                    Ok(log) => {
                        queue.push(log);
                        queue.len() >= QUEUE_BUFFER
                    }
                };

                if flush {
                    client.log(queue);
                    queue = Vec::<Log>::with_capacity(QUEUE_BUFFER);
                }
            }

            println!("Shutting down dlog ingest");
        });

        Self { sender }
    }

    pub fn log(&self, priority: Priority, message: String) {
        if let Err(err) = self.sender.send(Log::new(priority, message)) {
            println!("Failed to move log to sender: {}", err);
        }
    }
}
