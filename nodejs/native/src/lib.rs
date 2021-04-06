use native::models::Priority;
use neon::prelude::*;
use once_cell::sync::OnceCell;

mod extractor;

use crate::extractor::Extractor;

static LOGGER: OnceCell<native::Logger> = OnceCell::new();

fn logger<'a>() -> &'a native::Logger {
    match LOGGER.get() {
        None => panic!("You need to call configure() before using dlog"),
        Some(val) => val,
    }
}

fn configure(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let api_key = cx.argument::<JsString>(0)?.value(&mut cx);
    LOGGER.set(native::Logger::new(api_key)).unwrap();
    Ok(JsUndefined::new(&mut cx))
}

fn log(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    logger().log(Priority::Critical, cx.extract()?.value(&mut cx));
    Ok(JsUndefined::new(&mut cx))
}

fn error(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    logger().log(Priority::Critical, cx.extract()?.value(&mut cx));
    Ok(JsUndefined::new(&mut cx))
}

register_module!(mut cx, {
    cx.export_function("configure", configure)?;
    cx.export_function("log", log)?;
    cx.export_function("error", error)?;
    Ok(())
});
