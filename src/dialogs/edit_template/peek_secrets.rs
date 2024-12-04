use std::{collections::HashMap, rc::Rc};

use dioxus::prelude::*;

use dioxus_utils::DataState;
use serde::*;

use crate::{
    icons::*,
    views::{load_secrets, SecretListItemApiModel},
};

#[component]
pub fn PeekSecrets(yaml: String) -> Element {
    let mut component_state = use_signal(|| PeekSecretsState::new());

    let component_state_read_model = component_state.read();

    let loaded_secrets = match component_state_read_model.loaded_secrets.as_ref() {
        DataState::None => {
            spawn(async move {
                component_state.write().loaded_secrets = DataState::Loading;
                match load_secrets().await {
                    Ok(as_vec) => {
                        let mut values = HashMap::new();

                        for itm in as_vec {
                            values.insert(itm.name.clone(), itm);
                        }

                        component_state.write().loaded_secrets = DataState::Loaded(values);
                    }
                    Err(err) => {
                        component_state.write().loaded_secrets = DataState::Error(err.to_string());
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
            };
        }

        DataState::Loaded(data) => data,

        DataState::Error(err) => {
            return rsx! {
                div { {err.as_str()} }
            };
        }
    };

    let mut secrets_to_render = Vec::new();

    if yaml.len() > 10 {
        for secret_name in settings_utils::placeholders::get_secret_names(yaml.as_str()) {
            let secret_name_to_load = Rc::new(secret_name.to_string());

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
                match component_state_read_model.secrets_values.get(secret_name) {
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
                                        let secret_name = secret_name_to_load.clone();
                                        spawn(async move {
                                            let secret_model = load_secret(secret_name.to_string()).await;
                                            if let Ok(secret_model) = secret_model {
                                                if secret_model.name.as_str().len() > 0 {
                                                    component_state
                                                        .write()
                                                        .insert_secret_value(
                                                            secret_name.to_string(),
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

pub struct PeekSecretsState {
    pub loaded_secrets: DataState<HashMap<String, SecretListItemApiModel>>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SecretApiModel {
    pub name: String,
    pub value: String,
    pub level: i32,
}

#[server]
async fn load_secret(secret_name: String) -> Result<SecretApiModel, ServerFnError> {
    let secret_model =
        crate::server::grpc_client::SecretsGrpcClient::get_secret(secret_name.clone())
            .await
            .unwrap();

    let result = SecretApiModel {
        name: secret_name,
        value: secret_model.value,
        level: secret_model.level,
    };

    Ok(result)

    /*
    if secret_model.name.len() > 0 {
        secrets.modify(|itm| {
            let mut secrets = itm.clone();
            secrets.insert(secret_name, secret_model);
            secrets
        });
    }

    let secret_name = secret_name.to_string();
    let secrets = secrets.to_owned();

    cx.spawn(async move {});
     */
}
