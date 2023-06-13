use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn throw_error() -> Result<(), JsError> {
    Ok(hello::error::throw_error()?)
}
