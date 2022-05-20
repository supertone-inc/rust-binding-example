#[napi]
pub mod string {
    #[napi]
    pub fn to_uppercase(s: String) -> String {
        hello::string::to_uppercase(&s)
    }
}
