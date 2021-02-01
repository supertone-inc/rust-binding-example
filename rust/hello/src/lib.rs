pub fn print_string(name: &str) {
    println!("Hello {}!", name)
}

pub fn get_string(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_string() {
        print_string("world");
    }

    #[test]
    fn test_get_string() {
        assert_eq!(get_string("world"), "Hello world!");
    }
}
