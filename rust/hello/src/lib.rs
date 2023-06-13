pub mod array;
pub mod callback;
pub mod error;
pub mod string;
pub mod structure;

pub use error::Error;

#[cfg(test)]
#[macro_use]
extern crate assert_matches;
