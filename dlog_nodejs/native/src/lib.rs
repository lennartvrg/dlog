use neon::prelude::*;

mod extractor;

use crate::extractor::Extractor;
use dlog_core::transforms::Transforms;

pub struct Logger(dlog_core::Logger);

impl Finalize for Logger {}

fn clean_up(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    cx.context()?.0.clean_up();
    Ok(JsUndefined::new(&mut cx))
}

fn configure(mut cx: FunctionContext) -> JsResult<JsBox<Logger>> {
    let api_key = cx.argument::<JsString>(0)?.value(&mut cx);

    let options = cx.argument::<JsObject>(1)?;
    let sanitize_emails = options.get(&mut cx, "sanitize_emails").map_or(false, |kv| {
        kv.downcast::<JsBoolean, _>(&mut cx)
            .map_or(true, |kv| kv.value(&mut cx))
    });
    let sanitize_credit_cards = options.get(&mut cx, "sanitize_credit_cards").map_or(false, |kv| {
        kv.downcast::<JsBoolean, _>(&mut cx)
            .map_or(true, |kv| kv.value(&mut cx))
    });

    let mut transforms = Transforms::new();
    transforms.add_email_sanitizer(sanitize_emails);
    transforms.add_credit_card_sanitizer(sanitize_credit_cards);

    match dlog_core::Logger::new(api_key, transforms) {
        Err(err) => cx.throw_error(err),
        Ok(val) => Ok(cx.boxed(Logger(val))),
    }
}

fn log(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let level = cx.level()?;
    let message = cx.message()?.value(&mut cx);
    match cx.context()?.0.log(level, message) {
        Err(err) => cx.throw_error(err),
        Ok(_) => Ok(JsUndefined::new(&mut cx)),
    }
}

fn flush(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    match cx.context()?.0.flush() {
        Err(err) => cx.throw_error(err),
        Ok(_) => Ok(JsUndefined::new(&mut cx)),
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("configure", configure)?;
    cx.export_function("cleanUp", clean_up)?;
    cx.export_function("log", log)?;
    cx.export_function("flush", flush)?;

    Ok(())
}
