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

#[derive(Serialize)]
pub struct LogRequest<'a> {
    pub logs: &'a [Log],
}

impl<'a> LogRequest<'a> {
    pub fn new(logs: &'a [Log]) -> Self {
        Self { logs }
    }
}
