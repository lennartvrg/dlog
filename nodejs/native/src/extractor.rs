use neon::context::FunctionContext;
use neon::result::JsResult;
use neon::types::{JsArray, JsBox, JsString};

use crate::Logger;

pub trait Extractor {
    fn context(&mut self) -> JsResult<JsBox<Logger>>;

    fn extract(&mut self) -> JsResult<JsString>;
}

impl<'a> Extractor for FunctionContext<'a> {
    fn context(&mut self) -> JsResult<'_, JsBox<Logger>> {
        self.argument::<JsBox<Logger>>(0)
    }

    fn extract(&mut self) -> JsResult<JsString> {
        let mut parts = Vec::<String>::new();
        for arg in self.argument::<JsArray>(1)?.to_vec(self)? {
            match arg.downcast::<JsString, _>(self) {
                Ok(str) => parts.push(str.value(self)),
                Err(_) => println!("Logging objects is currently not supported. Please stringify them before logging."),
            };
        }
        Ok(JsString::new(self, parts.join(" ")))
    }
}
