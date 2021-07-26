use neon::prelude::*;

pub fn init_module<'a, C: Context<'a>>(c: &mut C) -> JsResult<'a, JsObject> {
    let exports = JsObject::new(c);

    let to_uppercase = JsFunction::new(c, to_uppercase)?;
    exports.set(c, "to_uppercase", to_uppercase)?;

    Ok(exports)
}

fn to_uppercase(mut cx: FunctionContext) -> JsResult<JsString> {
    let string = cx.argument::<JsString>(0)?;
    let string = string.value(&mut cx);

    Ok(cx.string(&hello::string::to_uppercase(&string)))
}