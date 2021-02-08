use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Default(&'static str),
}

pub fn to_uppercase(s: &str) -> String {
    String::from(s).to_uppercase()
}

pub fn concat(a: &[f32], b: &[f32]) -> Vec<f32> {
    [a, b].concat()
}

pub fn raise_error() -> Result<()> {
    Err(Error::Default("error raised from Rust!"))
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
        match raise_error() {
            Err(Error::Default(msg)) => assert_eq!(msg, "error raised from Rust!"),
            _ => panic!("wrong result"),
        }
    }
}
