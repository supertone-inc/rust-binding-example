pub struct Greeter {
    name: String,
}

impl Greeter {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string()
    }

    pub fn greet(&self, subject_name: &str) -> String {
        format!("Hello {}! I'm {}.", subject_name, self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let mut greeter = Greeter::new("greeter");
        assert_eq!(greeter.name(), "greeter");

        greeter.set_name("greater");
        assert_eq!(greeter.name(), "greater");
    }

    #[test]
    fn test_greet() {
        let greeter = Greeter::new("greeter");
        assert_eq!(greeter.greet("greetee"), "Hello greetee! I'm greeter.");
    }
}
