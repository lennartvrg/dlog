use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

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

impl Display for Priority {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Emergency => write!(f, "Emergency"),
            Self::Alert => write!(f, "Alert"),
            Self::Critical => write!(f, "Critical"),
            Self::Error => write!(f, "Error"),
            Self::Warning => write!(f, "Warning"),
            Self::Notice => write!(f, "Notice"),
            Self::Informational => write!(f, "Informational"),
            Self::Debug => write!(f, "Debug"),
            Self::Trace => write!(f, "Trace"),
            Self::None => write!(f, "None"),
        }
    }
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
