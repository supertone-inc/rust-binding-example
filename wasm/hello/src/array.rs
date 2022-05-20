use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn concat(a: &[f32], b: &[f32]) -> Vec<f32> {
    hello::array::concat(&a, &b)
}
