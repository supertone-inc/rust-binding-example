use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn to_uppercase(s: &str) -> String {
    hello::string::to_uppercase(s)
}
