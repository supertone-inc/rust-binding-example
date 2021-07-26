use super::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Default(&'static str),
}

pub fn raise_error() -> Result<()> {
    Err(Error::Default("error raised from Rust!"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raise_error() {
        assert_matches!(raise_error(), Err(Error::Default(msg)) if msg == "error raised from Rust!");
    }
}
