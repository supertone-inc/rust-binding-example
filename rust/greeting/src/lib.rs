pub fn greet() {
    println!("Hello world!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        greet();
    }
}
