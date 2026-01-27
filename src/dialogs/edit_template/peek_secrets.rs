use std::{collections::HashMap, rc::Rc};

use dioxus::prelude::*;

use dioxus_utils::*;

use crate::models::*;

#[component]
pub fn PeekSecrets(env_id: Rc<String>, product_id: String, yaml: String) -> Element {
    let mut cs = use_signal(|| PeekSecretsState::new());

    let cs_ra = cs.read();

    let loaded_secrets = match get_data(cs, &cs_ra, env_id.as_str(), product_id.as_str()) {
        Ok(items) => items,
        Err(err) => return err,
    };

    let mut secrets_to_render = Vec::new();

    if yaml.len() > 10 {
        for secret_name in settings_utils::placeholders::get_secret_names(yaml.as_str()) {
            let secret_id_to_load = Rc::new(secret_name.to_string());
            let product_id_to_load = Rc::new(product_id.clone());

            let env_id = env_id.clone();

            let (secret_value, secret_level) = if !loaded_secrets.contains_key(secret_name) {
                (
                    rsx! {
                        div {
                            span { class: "badge text-bg-danger", "Secret not found" }
                        }
                    },
                    rsx! {
                        div {}
                    },
                )
            } else {
                match cs_ra.secrets_values.get(secret_name) {
                    Some(value) => (
                        rsx! {
                            div { style: "font-size:12px; width:300px; height:32px; overflow-y: auto;",
                                "{value.value}"
                            }
                        },
                        rsx! {
                            div { style: "font-size:12px", "{value.level}" }
                        },
                    ),
                    None => (
                        rsx! {
                            div { class: "btn-group",
                                button {
                                    class: "btn btn-primary btn-sm",
                                    onclick: move |_| {
                                        let env_id = env_id.to_string();
                                        let secret_id = secret_id_to_load.to_string();
                                        let product_id = if product_id_to_load.len() == 0 {
                                            None
                                        } else {
                                            Some(product_id_to_load.to_string())
                                        };
                                        spawn(async move {
                                            let secret_model = crate::api::secrets::load_secret(
                                                    env_id,
                                                    product_id,
                                                    secret_id.to_string(),
                                                )
                                                .await;
                                            if let Ok(secret_model) = secret_model {
                                                if secret_model.secret_id.as_str().len() > 0 {
                                                    cs.write()
                                                        .insert_secret_value(
                                                            secret_id.to_string(),
                                                            secret_model.clone(),
                                                        );
                                                }
                                            }
                                        });
                                    },
                                    "Load"
                                }
                            }
                        },
                        rsx! {
                            div {}
                        },
                    ),
                }
            };

            secrets_to_render.push(rsx! {
                tr { style: "border-top: 1px solid lightgray",
                    td { style: "font-size:12px; border-right: 1px solid lightgray",
                        "{secret_name}:"
                    }
                    td { width: "100%", {secret_value} }
                    td { width: "30px", {secret_level} }
                }
            });
        }
    }

    rsx! {
        div { style: "height:65vh; overflow-y: auto;",
            table { class: "table table-striped",
                tr {
                    th { "secret" }
                    th { "value" }
                    th { "level" }
                }
                {secrets_to_render.into_iter()}
            }
        }
    }
}

pub fn get_data<'s>(
    mut cs: Signal<PeekSecretsState>,
    cs_ra: &'s PeekSecretsState,
    env_id: &str,
    product_id: &str,
) -> Result<&'s HashMap<String, SecretHttpModel>, Element> {
    match cs_ra.loaded_secrets.as_ref() {
        RenderState::None => {
            let env_id = env_id.to_string();
            let product_id = if product_id.len() == 0 {
                None
            } else {
                Some(product_id.to_string())
            };
            spawn(async move {
                cs.write().loaded_secrets.set_loading();
                match crate::api::secrets::load_secrets(env_id, product_id).await {
                    Ok(as_vec) => {
                        let mut values = HashMap::new();

                        for itm in as_vec {
                            values.insert(itm.secret_id.clone(), itm);
                        }

                        cs.write().loaded_secrets.set_loaded(values);
                    }
                    Err(err) => {
                        cs.write().loaded_secrets.set_error(err.to_string());
                    }
                }
            });
            return Err(crate::icons::loading_icon());
        }
        RenderState::Loading => {
            return Err(crate::icons::loading_icon());
        }

        RenderState::Loaded(data) => Ok(data),

        RenderState::Error(err) => {
            return Err(crate::icons::render_error(err));
        }
    }
}

pub struct PeekSecretsState {
    pub loaded_secrets: DataState<HashMap<String, SecretHttpModel>>,
    pub secrets_values: HashMap<String, SecretApiModel>,
}

impl PeekSecretsState {
    pub fn new() -> Self {
        Self {
            loaded_secrets: DataState::new(),
            secrets_values: HashMap::new(),
        }
    }

    pub fn insert_secret_value(&mut self, name: String, value: SecretApiModel) {
        self.secrets_values.insert(name, value);
    }
}
