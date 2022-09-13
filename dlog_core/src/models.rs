use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize)]
pub enum Priority {
    Critical,
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

impl Display for Priority {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Critical => write!(f, "Critical"),
            Self::Error => write!(f, "Error"),
            Self::Warning => write!(f, "Warning"),
            Self::Info => write!(f, "Informational"),
            Self::Debug => write!(f, "Debug"),
            Self::Trace => write!(f, "Trace"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,

    pub priority: Priority,

    pub text: String,
}

impl Log {
    pub fn new(priority: Priority, message: impl Into<String>) -> Self {
        Self {
            timestamp: OffsetDateTime::now_utc(),
            priority,
            text: message.into(),
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
