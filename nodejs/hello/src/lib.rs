mod array;
mod error;
mod string;

#[macro_use]
extern crate napi_derive;

use napi::{Env, JsObject, Result};

#[module_exports]
fn init(mut exports: JsObject, env: Env) -> Result<()> {
    exports.set_named_property("array", array::init_module(env)?)?;
    exports.set_named_property("error", error::init_module(env)?)?;
    exports.set_named_property("string", string::init_module(env)?)?;

    Ok(())
}
