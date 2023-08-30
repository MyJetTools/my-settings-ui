use std::rc::Rc;

pub struct LastEdited {
    pub secret: Option<Rc<String>>,
    pub template: Option<Rc<(String, String)>>,
}

impl LastEdited {
    pub fn new() -> Self {
        Self {
            secret: None,
            template: None,
        }
    }

    pub fn set_last_secret_edited(&mut self, secret: String) {
        self.secret = Some(Rc::new(secret));
    }

    pub fn get_secret(&self) -> Rc<String> {
        match &self.secret {
            Some(secret) => Rc::clone(secret),
            None => Rc::new("".to_string()),
        }
    }

    pub fn set_last_template_edited(&mut self, env: String, name: String) {
        self.template = Some(Rc::new((env, name)));
    }

    pub fn get_template(&self) -> Rc<(String, String)> {
        match &self.template {
            Some(secret) => Rc::clone(secret),
            None => Rc::new(("".to_string(), "".to_string())),
        }
    }
}
