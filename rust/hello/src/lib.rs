pub mod error;
pub mod string;

pub use error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub fn concat(a: &[f32], b: &[f32]) -> Vec<f32> {
    [a, b].concat()
}

#[cfg(test)]
#[macro_use]
extern crate assert_matches;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat() {
        assert_eq!(
            concat(&[1.0, 2.0], &[3.0, 4.0, 5.0]),
            [1.0, 2.0, 3.0, 4.0, 5.0]
        );
    }
}
