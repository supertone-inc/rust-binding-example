pub fn map<F, T, U>(items: &[T], mapper: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    items.iter().map(mapper).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        assert_eq!(map(&[1, 2, 3], |v| *v as f32 / 2.0), [0.5, 1.0, 1.5]);
    }
}
