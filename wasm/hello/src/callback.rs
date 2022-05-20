use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn map(items: Box<[JsValue]>, mapper: &js_sys::Function) -> Result<Vec<JsValue>, JsValue> {
    let this = JsValue::null();
    hello::callback::map(&items, |item| mapper.call1(&this, item))
        .into_iter()
        .collect()
}
