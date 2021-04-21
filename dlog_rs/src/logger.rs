use log::{Level, Metadata, Record};

use dlog_core::models::Priority;

pub struct DlogLogger {
    core: dlog_core::Logger,
}

impl DlogLogger {
    pub fn new(core: dlog_core::Logger) -> Self {
        Self {
            core,
        }
    }
}

impl log::Log for DlogLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if let Err(err) = self
            .core
            .log(convert_level(record.level()), record.args().to_string())
        {
            eprintln!("[dlog] Error during log: {}", err)
        }
    }

    fn flush(&self) {
        if let Err(err) = self.core.flush() {
            eprintln!("[dlog] Error during flush: {}", err)
        }
    }
}

fn convert_level(level: Level) -> Priority {
    match level {
        Level::Error => Priority::Error,
        Level::Warn => Priority::Warning,
        Level::Info => Priority::Informational,
        Level::Debug => Priority::Debug,
        Level::Trace => Priority::Trace,
    }
}
