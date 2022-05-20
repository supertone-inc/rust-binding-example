use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn concat(a: Vec<JsValue>, b: Vec<JsValue>) -> Vec<JsValue> {
    let a: Vec<f32> = a.iter().map(|v| v.unchecked_into_f64() as f32).collect();
    let b: Vec<f32> = b.iter().map(|v| v.unchecked_into_f64() as f32).collect();
    hello::array::concat(&a, &b)
        .into_iter()
        .map(|v| JsValue::from(v))
        .collect()
}
