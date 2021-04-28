#[macro_use]
extern crate napi_derive;

use napi::{
    CallContext, ContextlessResult, Env, JsNumber, JsObject, JsString, JsUndefined, Result,
};
use std::convert::TryInto;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Hello(#[from] hello::Error),
}

impl From<Error> for napi::Error {
    fn from(err: Error) -> Self {
        Self::from_reason(err.to_string())
    }
}

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
    exports.create_named_method("to_uppercase", to_uppercase)?;
    exports.create_named_method("concat", concat)?;
    exports.create_named_method("raise_error", raise_error)?;

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
    result
        .iter()
        .enumerate()
        .map(|(i, v)| array.set_element(i as u32, ctx.env.create_double(*v as f64)?))
        .collect::<Result<Vec<()>>>()?;

    Ok(array)
}

#[contextless_function]
fn raise_error(env: Env) -> ContextlessResult<JsUndefined> {
    hello::raise_error().map_err(Into::<Error>::into)?;
    env.get_undefined().map(Some)
}
