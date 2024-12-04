pub struct EditDomainMaskState {
    pub value_init: String,
    pub value: String,
}

impl EditDomainMaskState {
    pub fn new(value: &str) -> Self {
        Self {
            value_init: value.to_string(),
            value: value.to_string(),
        }
    }

    pub fn can_be_saved(&self) -> bool {
        self.value != self.value_init
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn set_value(&mut self, value: &str) {
        self.value = value.to_string();
    }
}
