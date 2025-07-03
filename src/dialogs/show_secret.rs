use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_utils::DataState;
use serde::*;

use crate::dialogs::*;
use crate::icons::*;

#[component]
pub fn ShowSecret(env_id: Rc<String>, secret: Rc<String>) -> Element {
    let mut component_state = use_signal(|| ShowSecretState::new());

    let component_state_read_access = component_state.read();

    let content = match component_state_read_access.value.as_ref() {
        DataState::None => {
            let env_id = env_id.clone();
            let secret_name = secret.clone();
            spawn(async move {
                component_state.write().value = DataState::Loading;
                match load_secret_value(env_id.to_string(), secret_name.to_string()).await {
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
                    value: value.as_str(),
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
async fn load_secret_value(
    env_id: String,
    secret_id: String,
) -> Result<SecretValueApiModel, ServerFnError> {
    use crate::server::secrets_grpc::*;
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    let response = ctx
        .secrets_grpc
        .get(GetSecretRequest { name: secret_id })
        .await
        .unwrap();

    let result = SecretValueApiModel {
        value: response.value,
    };

    Ok(result)
}
