use dioxus::prelude::*;

use dioxus_utils::DataState;
use serde::*;

use crate::views::icons::*;

use super::*;

#[component]
pub fn EditSecret(name: String, on_ok: EventHandler<EditSecretResult>) -> Element {
    let mut component_state = use_signal(|| EditSecretState::new(name.clone()));

    let component_state_read_access = component_state.read();

    match component_state_read_access.value_on_init.as_ref() {
        DataState::None => {
            spawn(async move {
                component_state.write().value_on_init = DataState::Loading;
                match load_secret(name).await {
                    Ok(value) => {
                        component_state.write().init_value(SecretValue {
                            value: value.value,
                            level: value.level.to_string(),
                        });
                    }
                    Err(err) => {
                        component_state.write().value_on_init = DataState::Error(err.to_string());
                    }
                };
            });

            return rsx! {
                LoadingIcon {}
            };
        }
        DataState::Loading => {
            return rsx! {
                LoadingIcon {}
            }
        }

        DataState::Loaded(_) => {}

        DataState::Error(err) => {
            return rsx! {
                div { {err.as_str()} }
            }
        }
    }

    let content = rsx! {

        div { class: "form-floating mb-3",
            input {
                class: "form-control",
                disabled: !component_state_read_access.new_secret,
                oninput: move |cx| {
                    component_state.write().name = cx.value();
                },
                value: component_state_read_access.name.as_str()
            }
            label { "Secret name" }
        }

        div { class: "form-floating mb-3",
            input {
                class: "form-control",
                oninput: move |cx| {
                    component_state.write().value.value = cx.value();
                },
                value: component_state_read_access.value.value.as_str()
            }
            label { "Secret value" }
        }

        div { class: "form-floating mb-3",
            input {
                class: "form-control",
                r#type: "number",
                oninput: move |cx| {
                    component_state.write().value.level = cx.value();
                },
                value: component_state_read_access.value.level.as_str()
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
                    disabled: component_state_read_access.save_button_is_disabled(),
                    onclick: move |_| {
                        let result = component_state.read().get_result();
                        on_ok.call(result);
                        consume_context::<Signal<DialogState>>().set(DialogState::None);
                    },
                    OkButtonIcon {}
                    "Save"
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SecretValueApiModel {
    pub value: String,
    pub level: i32,
}

#[server]
pub async fn load_secret<'s>(secret_id: String) -> Result<SecretValueApiModel, ServerFnError> {
    let response = crate::server::grpc_client::SecretsGrpcClient::get_secret(secret_id)
        .await
        .unwrap();

    let result = SecretValueApiModel {
        value: response.value,
        level: response.level,
    };

    Ok(result)
    /*
    let dialog_state = dialog_state.to_owned();

    cx.spawn(async move {

        dialog_state.modify(|itm| itm.set_loaded_values(response.value, response.level));
    }); */
}

pub struct EditSecretResult {
    pub name: String,
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

        return Self {
            new_secret,
            name,
            value_on_init: if new_secret {
                DataState::Loaded(value.clone())
            } else {
                DataState::None
            },
            value,
        };
    }

    pub fn init_value(&mut self, value: SecretValue) {
        self.value = value.clone();
        self.value_on_init = DataState::Loaded(value);
    }

    pub fn can_be_saved(&self) -> bool {
        if self.name.len() == 0 {
            return false;
        }

        if self.value.value.len() == 0 {
            return false;
        }

        let value_on_init = match self.value_on_init.as_ref() {
            DataState::Loaded(value) => value,
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
            name: self.name.clone(),
            value: self.value.value.clone(),
            level: self.value.level.parse().unwrap(),
        }
    }

    pub fn save_button_is_disabled(&self) -> bool {
        !self.can_be_saved()
    }
}
