use neon::context::FunctionContext;
use neon::result::JsResult;
use neon::types::{JsArray, JsString};

pub trait Extractor {
    fn extract(&mut self) -> JsResult<JsString>;
}

impl<'a> Extractor for FunctionContext<'a> {
    fn extract(&mut self) -> JsResult<JsString> {
        let mut parts = Vec::<String>::new();
        for arg in self.argument::<JsArray>(0)?.to_vec(self)? {
            match arg.downcast::<JsString, _>(self) {
                Ok(str) => parts.push(str.value(self)),
                Err(_) => println!("Logging objects is currently not supported. Please stringify them before logging."),
            };
        }
        Ok(JsString::new(self, parts.join(" ")))
    }
}
