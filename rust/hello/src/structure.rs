pub struct Counter {
    count: i32,
}

impl Counter {
    pub fn new(count: i32) -> Self {
        Self { count }
    }

    pub fn increase(&mut self, amount: i32) -> i32 {
        self.count += amount;
        self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let mut counter = Counter::new(1);
        assert_eq!(counter.increase(2), 3);
    }
}
