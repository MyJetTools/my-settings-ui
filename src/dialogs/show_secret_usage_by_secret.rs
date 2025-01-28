use std::rc::Rc;

use dioxus::prelude::*;

use dioxus_utils::DataState;
use serde::*;

use crate::{dialogs::*, views::icons::*};

#[component]
pub fn ShowSecretUsageBySecret(env_id: Rc<String>, secret: Rc<String>) -> Element {
    let mut component_state = use_signal(|| ShowSecretUsageBySecretState::new());

    let component_state_read_state = component_state.read();

    let values = match component_state_read_state.data.as_ref() {
        DataState::None => {
            let env_id = env_id.clone();
            let secret_id = secret.to_string();
            spawn(async move {
                match load_secret_usage_by_secret(env_id.to_string(), secret_id).await {
                    Ok(result) => {
                        component_state.write().data = DataState::Loaded(result);
                    }
                    Err(err) => {
                        component_state.write().data = DataState::Error(err.to_string());
                    }
                }
            });
            return rsx! {
                div {}
            };
        }
        DataState::Loading => {
            return rsx! {
                LoadingIcon {}
            }
        }
        DataState::Loaded(data) => data,
        DataState::Error(err) => {
            return rsx! {
                div { {err.as_str()} }
            }
        }
    };

    let to_render = values.into_iter().map(|itm| {
        let index = itm.value.find(secret.as_str());

        match index {
            Some(index) => {
                let left = &itm.value[..index];
                let mid = &secret;
                let right = &itm.value[index + mid.len()..];
                rsx! {
                    tr {
                        td { "{itm.name}:" }
                        td {
                            div { style: "color:gray; padding-left:5px",
                                "{left}"
                                span { style: "color:black", "{mid}" }
                                span { style: "color:gray", "{right}" }
                            }
                        }
                    }
                }
            }
            None => {
                rsx! {
                    tr {

                        td { "{itm.name}:" }
                        td {
                            div { style: "color:gray; padding-left:5px", " {itm.value}" }
                        }
                    }
                }
            }
        }
    });

    rsx! {

        DialogTemplate {
            header: format!("Usage of secret {}", secret.as_str()),
            width: "95%",
            content: rsx! {
                div { style: "text-align:left", class: "dialog-max-content", {to_render} }
            },
        }
    }
}

pub struct ShowSecretUsageBySecretState {
    pub data: DataState<Vec<SecretUsageBySecretApiModel>>,
}

impl ShowSecretUsageBySecretState {
    pub fn new() -> Self {
        Self {
            data: DataState::None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SecretUsageBySecretApiModel {
    pub name: String,
    pub value: String,
}

#[server]
async fn load_secret_usage_by_secret(
    env_id: String,
    secret_id: String,
) -> Result<Vec<SecretUsageBySecretApiModel>, ServerFnError> {
    use crate::server::secrets_grpc::*;
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    let response = ctx
        .secrets_grpc
        .get_secrets_usage(GetSecretsUsageRequest { name: secret_id })
        .await
        .unwrap();

    let result: Vec<_> = response
        .secrets
        .into_iter()
        .map(|itm| SecretUsageBySecretApiModel {
            name: itm.name,
            value: itm.value,
        })
        .collect();

    Ok(result)
}
