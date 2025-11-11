use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_utils::{DataState, RenderState};
use serde::*;

use crate::dialogs::*;
use crate::icons::*;

#[component]
pub fn ShowSecret(env_id: Rc<String>, secret: Rc<String>) -> Element {
    let mut component_state = use_signal(|| ShowSecretState::new());

    let component_state_read_access = component_state.read();

    let content = match component_state_read_access.value.as_ref() {
        RenderState::None => {
            let env_id = env_id.clone();
            let secret_name = secret.clone();
            spawn(async move {
                component_state.write().value.set_loading();
                match load_secret_value(env_id.to_string(), secret_name.to_string()).await {
                    Ok(value) => {
                        component_state.write().value.set_loaded(value.value);
                    }
                    Err(err) => {
                        component_state.write().value.set_error(err.to_string());
                    }
                }
            });
            rsx! {
                div {}
            }
        }
        RenderState::Loading => {
            rsx! {
                LoadingIcon {}
            }
        }
        RenderState::Loaded(value) => rsx! {
            div { class: "form-floating mb-3",
                input {
                    class: "form-control",
                    readonly: true,
                    value: value.as_str(),
                }
                label { "Secret value" }
            }
        },
        RenderState::Error(err) => {
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
            value: DataState::new(),
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
