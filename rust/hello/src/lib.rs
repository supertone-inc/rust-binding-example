pub fn greet() {
    greeting::greet()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        greet();
    }
}
