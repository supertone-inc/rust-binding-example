pub fn to_uppercase(s: &str) -> String {
    String::from(s).to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_uppercase() {
        assert_eq!(to_uppercase("Hello World!"), "HELLO WORLD!");
    }
}
