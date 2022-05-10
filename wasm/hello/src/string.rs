use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn to_uppercase(s: &str) -> String {
    hello::string::to_uppercase(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_to_uppercase() {
        assert_eq!(to_uppercase("Hello World!"), "HELLO WORLD!");
    }
}
