use std::collections::HashSet;

use rust_extensions::ShortString;

#[derive(Default)]
pub struct TemplatesState {
    pub selected: HashSet<String>,
}

impl TemplatesState {
    pub fn is_selected(&self, env: &str, template_id: &str) -> bool {
        let id = generate_id(env, template_id);

        self.selected.contains(id.as_str())
    }

    pub fn set_selected(&mut self, env: &str, template_id: &str, value: bool) {
        let id = generate_id(env, template_id);

        if value {
            self.selected.insert(id.to_string());
        } else {
            self.selected.remove(id.as_str());
        }
    }

    pub fn has_selected(&self) -> bool {
        self.selected.len() > 0
    }
}

fn generate_id(env: &str, template_id: &str) -> ShortString {
    let mut result = ShortString::new_empty();

    result.push_str(env);
    result.push('|');
    result.push_str(template_id);

    result
}
