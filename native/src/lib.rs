use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::thread::JoinHandle;

mod ingestor;
pub mod models;

use crate::ingestor::HttpIngestor;
use crate::models::{Log, Priority};

const QUEUE_BUFFER: usize = 1_000;

const MIN_FLUSH_INTERVAL: std::time::Duration = std::time::Duration::from_secs(1);

#[derive(Debug)]
pub struct Logger {
    sender: flume::Sender<Log>,
    flag: Arc<AtomicBool>,
    handle: RwLock<Option<JoinHandle<()>>>,
}

impl Logger {
    pub fn new(api_key: String) -> Self {
        let (sender, receiver) = flume::bounded::<Log>(QUEUE_BUFFER);

        let flag = Arc::new(AtomicBool::new(false));
        let thread_flag = flag.clone();

        let handle = std::thread::spawn(move || {
            let client = HttpIngestor::new(api_key);
            let mut queue = Vec::<Log>::with_capacity(QUEUE_BUFFER);

            while !thread_flag.load(Ordering::Relaxed) {
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

            client.log(queue);
        });

        Self {
            sender,
            flag,
            handle: RwLock::new(Some(handle)),
        }
    }

    pub fn log(&self, priority: Priority, message: String) -> Result<(), String> {
        match self.sender.send(Log::new(priority, message)) {
            Err(err) => Err(format!("Failed to move log to sender: {}", err)),
            Ok(_) => Ok(()),
        }
    }

    pub fn clean_up(&self) {
        self.flag.store(true, Ordering::Relaxed);

        let mut write = match self.handle.write() {
            Err(err) => {
                println!("Failed to get write lock during cleanup: {}", err);
                return
            },
            Ok(val) => val,
        };

        let handle = match write.take() {
            None => return,
            Some(val) => val,
        };

        if let Err(err) = handle.join() {
            println!("Failed to join queue thread: {:?}", err)
        }
    }
}
