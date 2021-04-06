use native::models::Priority;
use neon::prelude::*;

mod extractor;

use crate::extractor::Extractor;

pub struct Logger(native::Logger);

impl Finalize for Logger {}

fn clean_up(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    cx.context()?.0.clean_up();
    Ok(JsUndefined::new(&mut cx))
}

fn configure(mut cx: FunctionContext) -> JsResult<JsBox<Logger>> {
    let api_key = cx.argument::<JsString>(0)?.value(&mut cx);
    Ok(cx.boxed(Logger(native::Logger::new(api_key))))
}

fn log(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let message = cx.extract()?.value(&mut cx);
    match cx.context()?.0.log(Priority::Critical, message) {
        Err(err) => cx.throw_error(err),
        Ok(_) => Ok(JsUndefined::new(&mut cx)),
    }
}

fn error(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let message = cx.extract()?.value(&mut cx);
    match cx.context()?.0.log(Priority::Critical, message) {
        Err(err) => cx.throw_error(err),
        Ok(_) => Ok(JsUndefined::new(&mut cx)),
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("configure", configure)?;
    cx.export_function("cleanUp", clean_up)?;
    cx.export_function("log", log)?;
    cx.export_function("error", error)?;

    Ok(())
}
