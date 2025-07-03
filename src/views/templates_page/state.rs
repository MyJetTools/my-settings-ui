use std::collections::HashMap;

#[derive(Default)]
pub struct TemplatesState {
    pub selected: HashMap<String, ()>,
}

impl TemplatesState {
    pub fn is_selected(&self, env: &str, template_id: &str) -> bool {
        let id = generate_id(env, template_id);

        self.selected.contains_key(id.as_str())
    }

    pub fn set_selected(&mut self, env: &str, template_id: &str, value: bool) {
        let id = generate_id(env, template_id);

        if value {
            self.selected.insert(id.to_string(), ());
        } else {
            self.selected.remove(id.as_str());
        }
    }
}

fn generate_id(env: &str, template_id: &str) -> String {
    format!("{}/{}", env, template_id)
}
