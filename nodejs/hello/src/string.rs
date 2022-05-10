#[napi]
mod string {
    #[napi]
    fn to_uppercase(s: String) -> String {
        hello::string::to_uppercase(&s)
    }
}
