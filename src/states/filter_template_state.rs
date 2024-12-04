use crate::TemplateApiModel;

pub struct FilterTemplate(String);

impl FilterTemplate {
    pub fn new() -> Self {
        Self(String::new())
    }

    pub fn set_value(&mut self, value: &str) {
        self.0 = value.to_lowercase();
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn filter_record(&self, itm: &TemplateApiModel) -> bool {
        if self.0.len() == 0 {
            return true;
        }

        itm.name.to_lowercase().contains(self.0.as_str())
    }
}
