use log::{Level, Metadata, Record};

use dlog_core::models::Priority;

pub struct DlogLogger {
    core: dlog_core::Logger,
    level: Level,
}

impl DlogLogger {
    pub fn new(core: dlog_core::Logger, level: Level) -> Self {
        Self { core, level }
    }
}

impl log::Log for DlogLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() >= self.level
    }

    fn log(&self, record: &Record) {
        if let Err(err) = self.core.log(convert_level(record.level()), record.args().to_string()) {
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
        Level::Info => Priority::Info,
        Level::Debug => Priority::Debug,
        Level::Trace => Priority::Trace,
    }
}
