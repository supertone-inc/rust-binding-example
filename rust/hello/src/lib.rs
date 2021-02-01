pub fn greet() {
    println!("Hello!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        greet();
    }
}
