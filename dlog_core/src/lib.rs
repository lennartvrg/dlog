use std::sync::{RwLock};

mod ingestor;
pub mod models;
mod transforms;
mod worker;

use crate::models::{Log, Priority};
use crate::transforms::Transforms;
use crate::worker::{Signal, Worker};

const FLUSH_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);

#[derive(Debug)]
pub struct Logger {
    signal_sender: flume::Sender<Signal>,
    flush_receiver: flume::Receiver<()>,
    handle: RwLock<Option<tokio::runtime::Runtime>>,
}

impl Logger {
    pub fn new(api_key: String) -> Result<Self, String> {
        let (mut worker, signal_sender, flush_receiver) = Worker::new(api_key, Transforms::new())?;

        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.spawn(async move {
            worker.start().await;
        });

        Ok(Self {
            signal_sender,
            flush_receiver,
            handle: RwLock::new(Some(runtime)),
        })
    }

    pub fn log(&self, priority: Priority, message: String) -> Result<(), String> {
        match self.signal_sender.send(Signal::Log(Log::new(priority, message))) {
            Err(err) => Err(format!("Failed to move log to sender: {}", err)),
            _ => Ok(()),
        }
    }

    pub fn flush(&self) -> Result<(), String> {
        if let Err(err) = self.signal_sender.send(Signal::Flush) {
            return Err(format!("Failed to send thread signal: {}", err));
        }

        match self.flush_receiver.recv_timeout(FLUSH_TIMEOUT) {
            Err(flume::RecvTimeoutError::Disconnected) => Err("Failed to receive thread signal".to_string()),
            _ => Ok(()),
        }
    }

    pub fn clean_up(&self) {
        match self.signal_sender.send(Signal::Exit) {
            Err(err) => println!("[dlog] Could not send exit signal, some logs might be lost: {}", err),
            Ok(_) => {
                let _ = self.flush_receiver.recv_timeout(FLUSH_TIMEOUT);
            }
        }

        let mut write = match self.handle.write() {
            Err(err) => {
                println!("[dlog] Failed to get write lock during cleanup: {}", err);
                return;
            }
            Ok(val) => val,
        };

        let _ = write.take();
    }
}
