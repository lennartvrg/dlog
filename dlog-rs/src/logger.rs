use log::{Level, Metadata, Record};

use native::models::Priority;

pub struct DlogLogger {
    native: native::Logger,
}

impl DlogLogger {
    pub fn new(native: native::Logger) -> Self {
        Self {
            native,
        }
    }
}

impl log::Log for DlogLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if let Err(err) = self
            .native
            .log(convert_level(record.level()), record.args().to_string())
        {
            eprintln!("[dlog] Error during log: {}", err)
        }
    }

    fn flush(&self) {
        if let Err(err) = self.native.flush() {
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
