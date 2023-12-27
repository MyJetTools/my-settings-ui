pub struct FilterSecret(String);

impl FilterSecret {
    pub fn new() -> Self {
        Self(String::new())
    }

    pub fn set_value(&mut self, value: &str) {
        self.0 = value.to_lowercase();
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
