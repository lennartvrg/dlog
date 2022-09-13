use neon::context::FunctionContext;
use neon::result::{JsResult, Throw};
use neon::types::{JsArray, JsBox, JsNumber, JsString};

use dlog_core::models::Priority;

use crate::Logger;

pub trait Extractor {
    fn context(&mut self) -> JsResult<JsBox<Logger>>;

    fn level(&mut self) -> Result<Priority, Throw>;

    fn message(&mut self) -> Result<String, Throw>;
}

impl<'a> Extractor for FunctionContext<'a> {
    fn context(&mut self) -> JsResult<'_, JsBox<Logger>> {
        self.argument::<JsBox<Logger>>(0)
    }

    fn level(&mut self) -> Result<Priority, Throw> {
        Ok(match self.argument::<JsNumber>(1)?.value(self) as i32 {
            50 => Priority::Error,
            40 => Priority::Warning,
            30 => Priority::Info,
            20 => Priority::Debug,
            _ => Priority::Trace,
        })
    }

    fn message(&mut self) -> Result<String, Throw> {
        let mut parts = Vec::<String>::new();
        for arg in self.argument::<JsArray>(2)?.to_vec(self)? {
            match arg.downcast::<JsString, _>(self) {
                Ok(str) => parts.push(str.value(self)),
                Err(_) => println!("Logging objects is currently not supported. Please stringify them before logging."),
            };
        }
        Ok(parts.join(" "))
    }
}
