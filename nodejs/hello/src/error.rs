use thiserror::Error;

use napi::{ContextlessResult, Env, JsObject, JsUndefined, Result};

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

pub fn init_module(env: Env) -> Result<JsObject> {
    let mut exports: JsObject = env.create_object()?;

    exports.create_named_method("raise_error", raise_error)?;

    Ok(exports)
}

#[contextless_function]
fn raise_error(env: Env) -> ContextlessResult<JsUndefined> {
    hello::error::raise_error().map_err(Into::<Error>::into)?;
    env.get_undefined().map(Some)
}
