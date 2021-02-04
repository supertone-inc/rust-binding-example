#[macro_use]
extern crate error_chain;

pub mod errors {
    error_chain! {}
}

use errors::*;

pub fn to_uppercase(s: &str) -> String {
    String::from(s).to_uppercase()
}

pub fn concat(a: &[f32], b: &[f32]) -> Vec<f32> {
    [a, b].concat()
}

pub fn raise_error() -> Result<()> {
    bail!("error raised from Rust!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_uppercase() {
        assert_eq!(to_uppercase("Hello World!"), "HELLO WORLD!");
    }

    #[test]
    fn test_concat() {
        assert_eq!(
            concat(&[1.0, 2.0], &[3.0, 4.0, 5.0]),
            [1.0, 2.0, 3.0, 4.0, 5.0]
        );
    }

    #[test]
    fn test_raise_error() {
        assert_eq!(
            raise_error().err().unwrap().to_string(),
            "error raised from Rust!"
        );
    }
}
