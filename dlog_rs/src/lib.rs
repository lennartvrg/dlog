use log::LevelFilter;

use crate::logger::DlogLogger;

mod logger;

pub fn configure(api_key: impl Into<String>) {
    let native = match dlog_core::Logger::new(api_key.into()) {
        Err(err) => panic!("[dlog] Failed to configure dlog: {}", err),
        Ok(val) => val,
    };

    if let Err(err) = log::set_boxed_logger(Box::new(DlogLogger::new(native))) {
        panic!("[dlog] Failed to configure dlog: {}", err)
    }

    log::set_max_level(LevelFilter::Trace);
}