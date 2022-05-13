pub mod array;
pub mod callback;
pub mod error;
pub mod string;

pub use error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
#[macro_use]
extern crate assert_matches;
