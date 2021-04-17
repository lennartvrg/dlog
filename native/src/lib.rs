use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::thread::JoinHandle;

mod ingestor;
pub mod models;

use crate::ingestor::HttpIngestor;
use crate::models::{Log, Priority};

const QUEUE_BUFFER: usize = 1_000;

const MIN_FLUSH_INTERVAL: std::time::Duration = std::time::Duration::from_secs(1);
const MIN_LOOP_INTERVAL: std::time::Duration = std::time::Duration::from_millis(100);

#[derive(Debug)]
pub struct Logger {
    log_tx: flume::Sender<Log>,
    flag: Arc<AtomicBool>,
    handle: RwLock<Option<JoinHandle<()>>>,
}

impl Logger {
    pub fn new(api_key: String) -> Result<Self, String> {
        let (log_tx, log_rx) = flume::bounded::<Log>(QUEUE_BUFFER);

        let flag = Arc::new(AtomicBool::new(false));
        let thread_flag = flag.clone();

        let (thread_tx, thread_rx) = flume::bounded::<bool>(1);
        let handle = std::thread::spawn(move || {
            let client = HttpIngestor::new(api_key);

            let has_api_key = client.log(vec![
                Log::new(Priority::Trace, "Initialized dlog".to_owned())
            ]);

            if let Err(err) = thread_tx.send(has_api_key) {
                println!("[dlog] Failed to signal API_KEY check: {}", err);
            }

            if has_api_key {
                let mut queue = Vec::<Log>::with_capacity(QUEUE_BUFFER);

                let mut last_flush = std::time::Instant::now();
                while !thread_flag.load(Ordering::Relaxed) {
                    match log_rx.recv_timeout(MIN_LOOP_INTERVAL) {
                        Err(flume::RecvTimeoutError::Disconnected) => break,
                        Ok(log) => queue.push(log),
                        _ => (),
                    };
    
                    if queue.len() >= QUEUE_BUFFER || last_flush.elapsed() >= MIN_FLUSH_INTERVAL {
                        if queue.len() >= 1 {
                            client.log(queue);
                            queue = Vec::<Log>::with_capacity(QUEUE_BUFFER);
                        }
                        last_flush = std::time::Instant::now();
                    }
                }
    
                client.log(queue);
            }
        });

        match thread_rx.recv() {
            Err(err) => println!("[dlog::configure] Failed to receive API_KEY check signal: {}", err),
            Ok(false) => return Err("[dlog::configure] Please configure dlog with a valid API_KEY!".to_owned()),
            _ => (),
        };

        Ok(Self {
            log_tx,
            flag,
            handle: RwLock::new(Some(handle)),
        })
    }

    pub fn log(&self, priority: Priority, message: String) -> Result<(), String> {
        match self.log_tx.send(Log::new(priority, message)) {
            Err(err) => Err(format!("Failed to move log to sender: {}", err)),
            Ok(_) => Ok(()),
        }
    }

    pub fn clean_up(&self) {
        self.flag.store(true, Ordering::Relaxed);

        let mut write = match self.handle.write() {
            Err(err) => {
                println!("[dlog] Failed to get write lock during cleanup: {}", err);
                return
            },
            Ok(val) => val,
        };

        let handle = match write.take() {
            None => return,
            Some(val) => val,
        };

        if let Err(err) = handle.join() {
            println!("[dlog] Failed to join ingest thread: {:?}", err)
        }
    }
}
