use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn concat(a: Box<[JsValue]>, b: Box<[JsValue]>) -> Box<[JsValue]> {
    let a: Vec<f32> = a.iter().map(|v| v.unchecked_into_f64() as f32).collect();
    let b: Vec<f32> = b.iter().map(|v| v.unchecked_into_f64() as f32).collect();
    hello::array::concat(&a, &b)
        .iter()
        .map(|v| JsValue::from(*v))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_concat() {
        assert_eq!(
            concat(
                [1.0, 2.0].iter().map(|v| JsValue::from(*v)).collect(),
                [3.0, 4.0, 5.0].iter().map(|v| JsValue::from(*v)).collect()
            ),
            [1.0, 2.0, 3.0, 4.0, 5.0]
                .iter()
                .map(|v| JsValue::from(*v))
                .collect()
        );
    }
}
