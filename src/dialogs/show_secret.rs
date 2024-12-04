use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_utils::DataState;
use serde::*;

use crate::dialogs::*;
use crate::views::icons::*;

#[component]
pub fn ShowSecret(secret: Rc<String>) -> Element {
    let mut component_state = use_signal(|| ShowSecretState::new());

    let component_state_read_access = component_state.read();

    let content = match component_state_read_access.value.as_ref() {
        DataState::None => {
            let secret_name = secret.clone();
            spawn(async move {
                component_state.write().value = DataState::Loading;
                match load_secret(secret_name.to_string()).await {
                    Ok(value) => {
                        component_state.write().value = DataState::Loaded(value.value);
                    }
                    Err(err) => {
                        component_state.write().value = DataState::Error(err.to_string());
                    }
                }
            });
            rsx! {
                div {}
            }
        }
        DataState::Loading => {
            rsx! {
                LoadingIcon {}
            }
        }
        DataState::Loaded(value) => rsx! {
            div { class: "form-floating mb-3",
                input {
                    class: "form-control",
                    readonly: true,
                    value: value.as_str()
                }
                label { "Secret value" }
            }
        },
        DataState::Error(err) => {
            rsx! {
                div { {err.as_str()} }
            }
        }
    };

    rsx! {
        DialogTemplate { header: "Secret [{secret.as_str()}] value", content }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SecretValueApiModel {
    pub value: String,
}

pub struct ShowSecretState {
    pub value: DataState<String>,
}

impl ShowSecretState {
    pub fn new() -> Self {
        Self {
            value: DataState::None,
        }
    }
}

#[server]
async fn load_secret<'s>(secret_id: String) -> Result<SecretValueApiModel, ServerFnError> {
    let response = crate::server::grpc_client::SecretsGrpcClient::get_secret(secret_id)
        .await
        .unwrap();

    let result = SecretValueApiModel {
        value: response.value,
    };

    Ok(result)
}
