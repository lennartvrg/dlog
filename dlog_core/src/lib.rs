use std::sync::RwLock;

mod backlog;
mod ingest;
pub mod models;
pub mod transforms;
mod worker;

use crate::models::{Log, Priority};
use crate::transforms::Transforms;
use crate::worker::{Signal, Worker};

const FLUSH_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(3);

#[derive(Debug)]
pub struct Logger {
    signal_sender: flume::Sender<Signal>,
    flush_receiver: flume::Receiver<()>,
    handle: RwLock<Option<tokio::runtime::Runtime>>,
}

impl Logger {
    pub fn new(api_key: String, transforms: Transforms) -> Result<Self, String> {
        let (mut worker, mut backlog) = Worker::new(api_key, transforms)?;
        let (signal_sender, flush_receiver) = (worker.signal_sender.clone(), worker.flush_receiver.clone());

        let (valid_tx, valid_rx) = flume::bounded(1);
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.spawn(async move {
            if let Err(err) = valid_tx.send(worker.has_valid_api_key().await) {
                panic!(
                    "[dlog::logger] Internal error: The API_KEY channel is closed (Sending end) | {}",
                    err
                )
            }

            let _ = futures::future::try_join_all(vec![
                tokio::task::spawn(async move { worker.start().await }),
                tokio::task::spawn(async move { backlog.start().await }),
            ])
            .await;
        });

        match valid_rx.recv() {
            Err(err) => {
                runtime.shutdown_background();
                panic!(
                    "[dlog::logger] Internal error: The API_KEY channel is closed (Receiving end) | {}",
                    err
                )
            },
            Ok(false) => {
                runtime.shutdown_background();
                return Err(String::from(
                    "[dlog::logger] Please configure dlog with a valid API_KEY",
                ))
            }
            _ => (),
        };

        // Wait for first flush signal => Ready to be used
        if let Err(err) = flush_receiver.recv_timeout(std::time::Duration::from_secs(3)) {
            eprintln!("[dlog::logger] Failed to receive ready signal: {}", err);
        }

        Ok(Self {
            signal_sender,
            flush_receiver,
            handle: RwLock::new(Some(runtime)),
        })
    }

    pub fn log(&self, priority: Priority, message: String) -> Result<(), String> {
        match self.signal_sender.send(Signal::Log(Log::new(priority, message))) {
            Err(err) => Err(format!("[dlog::logger] Failed to move log to sender: {}", err)),
            _ => Ok(()),
        }
    }

    pub fn flush(&self) -> Result<(), String> {
        if let Err(err) = self.signal_sender.send(Signal::Flush) {
            return Err(format!("[dlog::logger] Failed to send thread signal: {}", err));
        }

        match self.flush_receiver.recv_timeout(FLUSH_TIMEOUT) {
            Err(flume::RecvTimeoutError::Disconnected) => {
                Err("[dlog::logger] Failed to receive thread signal".to_string())
            }
            _ => Ok(()),
        }
    }

    pub fn clean_up(&self) {
        match self.signal_sender.send(Signal::Exit) {
            Err(err) => println!(
                "[dlog::logger] Could not send exit signal, some logs might be lost: {}",
                err
            ),
            Ok(_) => {
                if let Err(err) = self.flush_receiver.recv_timeout(FLUSH_TIMEOUT) {
                    eprintln!("[dlog::logger] Failed to exit signal response: {}", err);
                }
            }
        }

        let mut write = match self.handle.write() {
            Err(err) => {
                println!("[dlog::logger] Failed to get write lock during cleanup: {}", err);
                return;
            }
            Ok(val) => val,
        };

        let _ = write.take();
    }
}
