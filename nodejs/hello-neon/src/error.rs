use neon::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Hello(#[from] hello::Error),
}

impl Error {
    pub fn to_throw<'a, C: Context<'a>>(&self, c: &mut C) -> neon::result::Throw {
        c.throw_error::<String, JsError>(self.to_string())
            .err()
            .unwrap()
    }
}

pub fn init_module<'a, C: Context<'a>>(c: &mut C) -> JsResult<'a, JsObject> {
    let exports = JsObject::new(c);

    let raise_error = JsFunction::new(c, raise_error)?;
    exports.set(c, "raise_error", raise_error)?;

    Ok(exports)
}

fn raise_error(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    hello::error::raise_error().map_err(|err| Error::from(err).to_throw(&mut cx))?;

    Ok(cx.undefined())
}
