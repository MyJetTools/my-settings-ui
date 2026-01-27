use std::rc::Rc;

use dioxus::prelude::*;

use dioxus_utils::*;

use crate::icons::*;

use super::*;

#[component]
pub fn EditSecret(
    env_id: Rc<String>,
    product_id: Option<Rc<String>>,
    secret_id: Rc<String>,
    on_ok: EventHandler<EditSecretResult>,
) -> Element {
    let mut cs = use_signal(|| EditSecretState::new(secret_id.to_string()));
    let cs_ra = cs.read();

    match get_data(cs, &cs_ra, &env_id, &product_id, &secret_id) {
        Ok(_) => {}
        Err(err) => return err,
    };

    let content = rsx! {

        div { class: "form-floating mb-3",
            input {
                class: "form-control",
                disabled: !cs_ra.new_secret,
                oninput: move |cx| {
                    cs.write().name = cx.value();
                },
                value: cs_ra.name.as_str(),
            }
            label { "Secret name" }
        }

        div { class: "form-floating mb-3",
            input {
                class: "form-control",
                oninput: move |cx| {
                    cs.write().value.value = cx.value();
                },
                value: cs_ra.value.value.as_str(),
            }
            label { "Secret value" }
        }

        div { class: "form-floating mb-3",
            input {
                class: "form-control",
                r#type: "number",
                oninput: move |cx| {
                    cs.write().value.level = cx.value();
                },
                value: cs_ra.value.level.as_str(),
            }
            label { "Secret level" }
        }
    };

    rsx! {

        DialogTemplate {
            header: "Edit secret",
            content,
            ok_button: rsx! {
                button {
                    class: "btn btn-primary",
                    disabled: cs_ra.save_button_is_disabled(),
                    onclick: move |_| {
                        let result = cs.read().get_result();
                        on_ok.call(result);
                        consume_context::<Signal<DialogState>>().set(DialogState::None);
                    },
                    OkButtonIcon {}
                    "Save"
                }
            },
        }
    }
}

fn get_data(
    mut cs: Signal<EditSecretState>,
    cs_ra: &EditSecretState,
    env_id: &str,
    product_id: &Option<Rc<String>>,
    secret_id: &str,
) -> Result<(), Element> {
    match cs_ra.value_on_init.as_ref() {
        RenderState::None => {
            let env_id = env_id.to_string();
            let product_id = match product_id {
                Some(product_id) => Some(product_id.to_string()),
                None => None,
            };
            let secret_id = secret_id.to_string();
            spawn(async move {
                cs.write().value_on_init.set_loading();
                match crate::api::secrets::load_secret_value(env_id, product_id, secret_id).await {
                    Ok(value) => {
                        cs.write().init_value(SecretValue {
                            value: value.value,
                            level: value.level.to_string(),
                        });
                    }
                    Err(err) => {
                        cs.write().value_on_init.set_error(err.to_string());
                    }
                };
            });

            return Err(crate::icons::loading_icon());
        }
        RenderState::Loading => {
            return Err(crate::icons::loading_icon());
        }

        RenderState::Loaded(_) => {
            return Ok(());
        }

        RenderState::Error(err) => {
            return Err(crate::icons::render_error(err));
        }
    }
}

pub struct EditSecretResult {
    pub secret_id: String,
    pub value: String,
    pub level: i32,
}

#[derive(Debug, Clone, Default)]
pub struct SecretValue {
    pub value: String,
    pub level: String,
}

pub struct EditSecretState {
    pub name: String,
    pub value: SecretValue,
    pub value_on_init: DataState<SecretValue>,
    pub new_secret: bool,
}

impl EditSecretState {
    pub fn new(name: String) -> Self {
        let new_secret = name.len() == 0;

        let value = SecretValue::default();

        let value_on_init = if new_secret {
            DataState::new_as_loaded(value.clone())
        } else {
            DataState::new()
        };

        return Self {
            new_secret,
            name,
            value_on_init,
            value,
        };
    }

    pub fn init_value(&mut self, value: SecretValue) {
        self.value = value.clone();
        self.value_on_init.set_loaded(value);
    }

    pub fn can_be_saved(&self) -> bool {
        if self.name.len() == 0 {
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

    pub fn get_result(&self) -> EditSecretResult {
        EditSecretResult {
            secret_id: self.name.clone(),
            value: self.value.value.clone(),
            level: self.value.level.parse().unwrap(),
        }
    }

    pub fn save_button_is_disabled(&self) -> bool {
        !self.can_be_saved()
    }
}
