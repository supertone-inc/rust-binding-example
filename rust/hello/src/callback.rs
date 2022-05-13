pub fn map<F, T>(items: &[T], mapper: F) -> Vec<T>
where
    F: Fn(&T) -> T,
{
    items.iter().map(mapper).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        assert_eq!(map(&[1, 2, 3], |v| v * 2), [2, 4, 6]);
    }
}
