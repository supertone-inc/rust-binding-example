#[macro_use]
extern crate napi_derive;

use std::convert::TryInto;

use napi::{CallContext, JsNumber, JsObject, JsString, Result};

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("to_uppercase", to_uppercase)?;
  exports.create_named_method("concat", concat)?;

  Ok(())
}

#[js_function(1)]
fn to_uppercase(ctx: CallContext) -> Result<JsString> {
  let s = ctx.get::<JsString>(0)?.into_utf8()?;

  ctx.env.create_string(&hello::to_uppercase(s.as_str()?))
}

#[js_function(2)]
fn concat(ctx: CallContext) -> Result<JsObject> {
  let a_obj: JsObject = ctx.get(0)?;
  let a_len: u32 = a_obj.get_array_length()?;
  let a = (0..a_len)
    .map(|i| {
      let value: f64 = a_obj.get_element::<JsNumber>(i)?.try_into()?;
      Ok(value as f32)
    })
    .collect::<Result<Vec<f32>>>()?;

  let b_obj: JsObject = ctx.get(1)?;
  let b_len: u32 = b_obj.get_array_length()?;
  let b = (0..b_len)
    .map(|i| {
      let value: f64 = b_obj.get_element::<JsNumber>(i)?.try_into()?;
      Ok(value as f32)
    })
    .collect::<Result<Vec<f32>>>()?;

  let result: Vec<f32> = hello::concat(&a, &b);
  let mut array = ctx.env.create_array()?;
  result.iter().enumerate()
    .map(|(i, v)| array.set_element(i as u32, ctx.env.create_double(*v as f64)?))
    .collect::<Result<Vec<()>>>()?;

  Ok(array)
}
