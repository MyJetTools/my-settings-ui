#[derive(Debug, Default)]
pub struct DialogValue<T: Clone + Eq + Default> {
    init_value: T,
    value: T,
}

impl<T: Clone + Eq + Default> DialogValue<T> {
    pub fn new(value: T) -> Self {
        Self {
            init_value: value.clone(),
            value,
        }
    }

    pub fn init(&mut self, value: T) {
        self.init_value = value.clone();
        self.value = value;
    }

    pub fn set_value(&mut self, value: T) {
        self.value = value;
    }

    pub fn get_init_value(&self) -> &T {
        &self.init_value
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn is_value_updated(&self) -> bool {
        self.init_value != self.value
    }

    pub fn get_value_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl DialogValue<String> {
    pub fn as_str(&self) -> &str {
        &self.value
    }
}
