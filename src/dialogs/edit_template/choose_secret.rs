use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_utils::*;

use crate::models::*;

use super::*;

#[component]
pub fn ChooseSecret(
    env_id: Rc<String>,
    product_id: String,
    on_selected: EventHandler<String>,
) -> Element {
    let mut cs = use_signal(|| ChooseSecretState::new());

    let cs_ra = cs.read();

    let content = match cs_ra.mode {
        ChooseSecretMode::Select => {
            let secrets = match get_data(cs, &cs_ra, &env_id, product_id.as_str()) {
                Ok(data) => data,
                Err(err) => return err,
            };

            let result = secrets
                .into_iter()
                .filter(|item| cs_ra.filter_it(item))
                .map(|value| {
                    let value = value.clone();
                    rsx! {
                        div {
                            button {
                                class: "btn btn-sm btn-primary",
                                onclick: move |_| {
                                    on_selected.call(value.secret_id.to_string());
                                },
                                "Copy"
                            }
                            "{value.secret_id}"
                        }
                    }
                });

            rsx! {
                {result}
            }
        }
        ChooseSecretMode::Add => {
            let product_id_to_add = product_id.clone();
            let btn = if cs_ra.has_secret() {
                rsx! {
                    div { class: "alert alert-danger", "Secret already exists" }
                }
            } else {
                let env_id_add_new_secret = env_id.clone();
                let product_id_new_secret = product_id.to_string();
                rsx! {
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            let env_id = env_id_add_new_secret.clone();
                            let product_id = if product_id_new_secret.len() == 0 {
                                None
                            } else {
                                Some(product_id_new_secret.to_string())
                            };
                            let (secret_id, secret_value, secret_level) = {
                                let component_state_read_access = cs.read();
                                (
                                    component_state_read_access.secret_name.clone(),
                                    component_state_read_access.secret_value.clone(),
                                    component_state_read_access.get_secret_level(),
                                )
                            };
                            spawn(async move {
                                let value = UpdateSecretValueHttpModel {
                                    product_id,
                                    secret_id,
                                    value: secret_value,
                                    level: secret_level,
                                };
                                crate::api::secrets::save_secret(env_id.to_string(), value).await.unwrap();
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
                        value: cs_ra.secret_value.as_str(),
                        oninput: move |cx| {
                            cs.write().secret_value = cx.value();
                        },
                    }
                    label { "Secret value" }
                }
                div { class: "form-floating mb-3",
                    input {
                        class: "form-control",
                        r#type: "number",
                        value: cs_ra.secret_level.as_str(),
                        oninput: move |cx| {
                            cs.write().secret_level = cx.value();
                        },
                    }
                    label { "Secret level" }
                }
                {btn}
                hr {}
                h4 { "Copy value from other secret" }
                SelectSecret {
                    env_id: env_id.clone(),
                    product_id: product_id.as_str(),
                    on_selected: move |value: String| {
                        let env_id = env_id.clone();

                        let product_id = if product_id_to_add.len() == 0 {
                            None
                        } else {
                            Some(product_id_to_add.clone())
                        };
                        let value = value.to_string();
                        spawn(async move {
                            let result = crate::api::secrets::load_secret(
                                    env_id.to_string(),
                                    product_id,
                                    value,
                                )
                                .await
                                .unwrap();
                            cs.write().secret_value = result.value;
                        });
                    },
                }
            }
        }
    };

    let btn = match cs_ra.mode {
        ChooseSecretMode::Select => {
            rsx! {
                button {
                    class: "btn btn-outline-secondary",
                    style: "width:150px",
                    onclick: move |_| {
                        cs.write().mode = ChooseSecretMode::Add;
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
                        cs.write().mode = ChooseSecretMode::Select;
                    },
                    "Add secret"
                }
            }
        }
    };

    let text = match cs_ra.mode {
        ChooseSecretMode::Select => "Search secret",
        ChooseSecretMode::Add => "Add secret",
    };

    rsx! {
        div { style: "margin-top:5px; width:100%", class: "input-group",
            input {
                class: "form-control",
                placeholder: text,
                oninput: move |cx| {
                    let mut write_access = cs.write();
                    write_access.secret_name = cx.value();
                    write_access.filter = write_access.secret_name.to_lowercase();
                },
            }
            {btn}
        }
        div { style: "height:65vh; overflow-y: auto; text-align: left", {content.into_iter()} }
    }
}

fn get_data<'s>(
    mut cs: Signal<ChooseSecretState>,
    cs_ra: &'s ChooseSecretState,
    env_id: &str,
    product_id: &str,
) -> Result<&'s [Rc<SecretHttpModel>], Element> {
    match cs_ra.secrets.as_ref() {
        RenderState::None => {
            let env_id = env_id.to_string();
            let product_id = product_id.to_string();

            spawn(async move {
                cs.write().secrets.set_loading();
                match crate::api::secrets::load_secrets(env_id, product_id).await {
                    Ok(secrets) => {
                        cs.write()
                            .secrets
                            .set_loaded(secrets.into_iter().map(Rc::new).collect());
                    }
                    Err(err) => {
                        cs.write().secrets.set_error(err.to_string());
                    }
                }
            });

            return Err(crate::icons::loading_icon());
        }
        RenderState::Loading => {
            return Err(crate::icons::loading_icon());
        }
        RenderState::Loaded(secrets) => {
            return Ok(secrets.as_slice());
        }
        RenderState::Error(err) => {
            return Err(crate::icons::render_error(err));
        }
    };
}

pub struct ChooseSecretState {
    pub filter: String,
    pub secret_name: String,
    pub secret_value: String,
    pub secret_level: String,
    pub mode: ChooseSecretMode,
    pub secrets: DataState<Vec<Rc<SecretHttpModel>>>,
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

    pub fn filter_it(&self, item: &SecretHttpModel) -> bool {
        if self.filter.len() < 3 {
            return false;
        }
        item.secret_id.to_lowercase().contains(self.filter.as_str())
    }

    pub fn get_secret_level(&self) -> i32 {
        self.secret_level.parse().unwrap()
    }

    pub fn has_secret(&self) -> bool {
        let secrets = self.secrets.unwrap_as_loaded();

        for value in secrets {
            if rust_extensions::str_utils::compare_strings_case_insensitive(
                value.secret_id.as_str(),
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
