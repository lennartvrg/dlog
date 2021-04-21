use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Priority {
    Emergency,
    Alert,
    Critical,
    Error,
    Warning,
    Notice,
    Informational,
    Debug,
    Trace,
    None,
}

#[derive(Serialize, Deserialize)]
pub struct Log {
    pub timestamp: DateTime<Utc>,

    pub priority: Priority,

    pub message: String,
}

impl Log {
    pub fn new(priority: Priority, message: String) -> Self {
        Self {
            timestamp: Utc::now(),
            priority,
            message,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LogRequest {
    pub logs: Vec<Log>,
}

impl LogRequest {
    pub fn new(logs: Vec<Log>) -> Self {
        Self {
            logs,
        }
    }
}