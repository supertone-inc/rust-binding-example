pub fn hello() {
    println!("Hello world!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        hello();
    }
}
