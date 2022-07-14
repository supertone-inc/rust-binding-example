use wasm_bindgen::prelude::*;

pub type Result<T, E = JsError> = std::result::Result<T, E>;

#[wasm_bindgen]
pub fn throw_error() -> Result<()> {
    Ok(hello::error::throw_error()?)
}
