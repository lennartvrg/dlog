#![crate_name = "dlog_rs"]
use log::Level;

use crate::logger::DlogLogger;

mod logger;

/// Configures dlog with the given API_KEY and sensitive default values
///
/// # Arguments
///
/// * `api_key` - The dlog API_KEY for this service.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate log;
///
/// fn main() {
///     dlog_rs::configure("<API_KEY>");
///
///     info!("Hello from Rust!");
///
///     // Flushes all remaining logs when the app exits
///     log::logger().flush();
/// }
/// ```
pub fn configure(api_key: impl Into<String>) {
    Builder::new().with_str_api_key(api_key).build();
}

/// The builder can be used to more finely configure dlog to best suit your needs.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate log;
///
/// use dlog_rs::Builder;
///
/// fn main() {
///     Builder::new()
///         .with_env_api_key("DLOG_API_KEY")
///         .with_level(log::Level::Trace)
///         .build();
///
///     info!("Hello from Rust!");
///
///     // Flushes all remaining logs when the app exits
///     log::logger().flush();
/// }
/// ```
pub struct Builder {
    api_key: Option<String>,
    level: Option<Level>,
}

impl Builder {
    /// Instantiates a new builder which can be used to configure dlog.
    pub fn new() -> Self {
        Self {
            api_key: None,
            level: None,
        }
    }

    /// Injects the API_KEY directly into the builder.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The dlog API_KEY for this service.
    pub fn with_str_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Injects the API_KEY into the builder by reading it from an environmental variable.
    ///
    /// # Arguments
    ///
    /// * `env_var` - The name of the environmental variable where this API_KEY is stored
    pub fn with_env_api_key(mut self, env_var: impl Into<String>) -> Self {
        self.api_key = Some(std::env::var(env_var.into()).unwrap_or_default());
        self
    }

    /// Sets the minimum level a log must have to be logged to dlog.
    ///
    /// # Arguments
    ///
    /// * `level` - The minimum required log level
    pub fn with_level(mut self, level: Level) -> Self {
        self.level = Some(level);
        self
    }

    /// Consumes the builder and configures dlog according to the builders configuration.
    pub fn build(self) {
        let native = match dlog_core::Logger::new(self.api_key.unwrap_or_default()) {
            Err(err) => panic!("[dlog] Failed to configure dlog: {}", err),
            Ok(val) => val,
        };

        let level = self.level.unwrap_or(Level::Debug);
        log::set_max_level(level.to_level_filter());

        let logger = DlogLogger::new(native, level);
        if let Err(err) = log::set_boxed_logger(Box::new(logger)) {
            panic!("[dlog] Failed to configure dlog: {}", err)
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}
