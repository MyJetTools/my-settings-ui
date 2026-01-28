use std::rc::Rc;

use dioxus_utils::*;

use crate::models::*;

pub struct EditSecretState {
    pub product_id: Option<String>,
    pub secret_id: String,
    pub value: SecretValue,
    pub value_on_init: DataState<SecretValue>,
    pub new_secret: bool,
}

impl EditSecretState {
    pub fn new(secret_id: String, product_id: &Option<Rc<String>>) -> Self {
        let new_secret = secret_id.len() == 0;

        let value = SecretValue::default();

        let value_on_init = if new_secret {
            DataState::new_as_loaded(value.clone())
        } else {
            DataState::new()
        };

        Self {
            new_secret,
            product_id: product_id.as_ref().map(|itm| itm.to_string()),
            secret_id,
            value_on_init,
            value,
        }
    }

    pub fn init_value(&mut self, value: SecretValue) {
        self.value = value.clone();
        self.value_on_init.set_loaded(value);
    }

    pub fn can_be_saved(&self) -> bool {
        if self.secret_id.len() == 0 {
            return false;
        }

        if self.value.value.len() == 0 {
            return false;
        }

        let value_on_init = match self.value_on_init.as_ref() {
            RenderState::Loaded(value) => value,
            _ => {
                return false;
            }
        };

        if self.value.value == value_on_init.value && self.value.level == value_on_init.level {
            return false;
        }

        true
    }

    pub fn get_result(&self) -> UpdateSecretValueHttpModel {
        UpdateSecretValueHttpModel {
            product_id: self.product_id.as_ref().map(|itm| itm.to_string()),
            secret_id: self.secret_id.clone(),
            value: self.value.value.clone(),
            level: self.value.level.parse().unwrap(),
        }
    }

    pub fn save_button_is_disabled(&self) -> bool {
        !self.can_be_saved()
    }
}

#[derive(Debug, Clone, Default)]
pub struct SecretValue {
    pub value: String,
    pub level: String,
}
