#[napi]
mod error {
    use napi::bindgen_prelude::*;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum Error {
        #[error(transparent)]
        Hello(#[from] hello::Error),
    }

    impl From<Error> for napi::Error {
        fn from(err: Error) -> Self {
            Self::from_reason(err.to_string())
        }
    }

    #[napi]
    fn raise_error() -> Result<()> {
        Ok(hello::error::raise_error().map_err(Into::<Error>::into)?)
    }
}
