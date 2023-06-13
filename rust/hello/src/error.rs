use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Default(&'static str),
}

pub fn throw_error() -> Result<(), Error> {
    Err(Error::Default("error from Rust!"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_throw_error() {
        assert_matches!(throw_error(), Err(Error::Default("error from Rust!")));
    }
}
