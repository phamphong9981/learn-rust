pub struct User {
    name: String,
    age: u8,
}

impl User {
    pub fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }
}
