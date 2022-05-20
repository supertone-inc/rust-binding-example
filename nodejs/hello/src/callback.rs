#[napi]
pub mod callback {
    use napi::*;

    #[napi]
    pub fn map(items: Vec<JsUnknown>, mapper: JsFunction) -> Result<Vec<JsUnknown>> {
        hello::callback::map(&items, |item| mapper.call(None, &[item]))
            .into_iter()
            .collect()
    }
}
