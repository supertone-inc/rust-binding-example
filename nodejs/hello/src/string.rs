use napi::{CallContext, Env, JsObject, JsString, Result};

pub fn init_module(env: Env) -> Result<JsObject> {
    let mut exports: JsObject = env.create_object()?;

    exports.create_named_method("to_uppercase", to_uppercase)?;

    Ok(exports)
}

#[js_function(1)]
fn to_uppercase(ctx: CallContext) -> Result<JsString> {
    let s = ctx.get::<JsString>(0)?.into_utf8()?;

    ctx.env
        .create_string(&hello::string::to_uppercase(s.as_str()?))
}
