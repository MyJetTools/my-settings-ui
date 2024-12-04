use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_utils::DataState;

use crate::views::icons::*;

use super::*;
use crate::dialogs::*;
use crate::save_secret;
use crate::views::{load_secrets, SecretListItemApiModel};

#[component]
pub fn ChooseSecret(on_selected: EventHandler<String>) -> Element {
    let mut component_state = use_signal(|| ChooseSecretState::new());

    let component_state_read_access = component_state.read();

    let content = match component_state_read_access.mode {
        ChooseSecretMode::Select => {
            let secrets = match component_state_read_access.secrets.as_ref() {
                DataState::None => {
                    spawn(async move {
                        component_state.write().secrets = DataState::Loading;
                        match load_secrets().await {
                            Ok(secrets) => {
                                component_state.write().secrets =
                                    DataState::Loaded(secrets.into_iter().map(Rc::new).collect());
                            }
                            Err(err) => {
                                component_state.write().secrets = DataState::Error(err.to_string());
                            }
                        }
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
                DataState::Loaded(secrets) => secrets,
                DataState::Error(err) => {
                    return rsx! {
                        div { {err.as_str()} }
                    }
                }
            };

            let result = secrets
                .into_iter()
                .filter(|item| component_state_read_access.filter_it(item))
                .map(|value| {
                    let value = value.clone();
                    rsx! {
                        div {
                            button {
                                class: "btn btn-sm btn-primary",
                                onclick: move |_| {
                                    on_selected.call(value.name.to_string());
                                },
                                "Copy"
                            }
                            "{value.name}"
                        }
                    }
                });

            rsx! {
                {result}
            }
        }
        ChooseSecretMode::Add => {
            let btn = if component_state_read_access.has_secret() {
                rsx! {
                    div { class: "alert alert-danger", "Secret already exists" }
                }
            } else {
                rsx! {
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            let (secret_name, secret_value, secret_level) = {
                                let component_state_read_access = component_state.read();
                                (
                                    component_state_read_access.secret_name.clone(),
                                    component_state_read_access.secret_value.clone(),
                                    component_state_read_access.get_secret_level(),
                                )
                            };
                            spawn(async move {
                                save_secret(secret_name, secret_value, secret_level).await.unwrap();
                            });
                        },
                        "Add new secret"
                    }
                }
            };
            rsx! {
                div { style: "margin-top:10px", class: "form-floating mb-3",
                    input {
                        class: "form-control",
                        value: component_state_read_access.secret_value.as_str(),
                        oninput: move |cx| {
                            component_state.write().secret_value = cx.value();
                        }
                    }
                    label { "Secret value" }
                }
                div { class: "form-floating mb-3",
                    input {
                        class: "form-control",
                        r#type: "number",
                        value: component_state_read_access.secret_level.as_str(),
                        oninput: move |cx| {
                            component_state.write().secret_level = cx.value();
                        }
                    }
                    label { "Secret level" }
                }
                {btn},
                hr {}
                h4 { "Copy value from other secret" }
                SelectSecret {
                    on_selected: move |value: String| {
                        spawn(async move {
                            let result = load_secret(value).await.unwrap();
                            component_state.write().secret_value = result.value;
                        });
                    }
                }
            }
        }
    };

    let btn = match component_state_read_access.mode {
        ChooseSecretMode::Select => {
            rsx! {
                button {
                    class: "btn btn-outline-secondary",
                    style: "width:150px",
                    onclick: move |_| {
                        component_state.write().mode = ChooseSecretMode::Add;
                    },
                    "Select secret"
                }
            }
        }
        ChooseSecretMode::Add => {
            rsx! {
                button {
                    class: "btn btn-outline-secondary",
                    style: "width:150px",
                    onclick: move |_| {
                        component_state.write().mode = ChooseSecretMode::Select;
                    },
                    "Add secret"
                }
            }
        }
    };

    let text = match component_state_read_access.mode {
        ChooseSecretMode::Select => "Search secret",
        ChooseSecretMode::Add => "Add secret",
    };

    rsx! {
        div { style: "margin-top:5px; width:100%", class: "input-group",
            input {
                class: "form-control",
                placeholder: text,
                oninput: move |cx| {
                    let mut write_access = component_state.write();
                    write_access.secret_name = cx.value();
                    write_access.filter = write_access.secret_name.to_lowercase();
                }
            }
            {btn}
        }
        div { style: "height:65vh; overflow-y: auto; text-align: left", {content.into_iter()} }
    }
}

pub struct ChooseSecretState {
    pub filter: String,
    pub secret_name: String,
    pub secret_value: String,
    pub secret_level: String,
    pub mode: ChooseSecretMode,
    pub secrets: DataState<Vec<Rc<SecretListItemApiModel>>>,
}

impl ChooseSecretState {
    pub fn new() -> Self {
        Self {
            filter: String::new(),
            secret_name: String::new(),
            mode: ChooseSecretMode::Select,
            secret_value: String::new(),
            secret_level: String::new(),
            secrets: DataState::new(),
        }
    }

    pub fn filter_it(&self, item: &SecretListItemApiModel) -> bool {
        if self.filter.len() < 3 {
            return false;
        }
        item.name.to_lowercase().contains(self.filter.as_str())
    }

    pub fn get_secret_level(&self) -> i32 {
        self.secret_level.parse().unwrap()
    }

    pub fn has_secret(&self) -> bool {
        let secrets = self.secrets.unwrap_as_loaded();

        for value in secrets {
            if rust_extensions::str_utils::compare_strings_case_insensitive(
                value.name.as_str(),
                &self.secret_name,
            ) {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ChooseSecretMode {
    Select,
    Add,
}
