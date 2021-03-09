#[macro_use]
extern crate napi_derive;

use napi::{CallContext, JsObject, Result, JsString};

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("to_uppercase", to_uppercase)?;

  Ok(())
}

#[js_function(1)]
fn to_uppercase(ctx: CallContext) -> Result<JsString> {
  let s = ctx.get::<JsString>(0)?.into_utf8()?;

  ctx.env.create_string(&hello::to_uppercase(s.as_str()?))
}
