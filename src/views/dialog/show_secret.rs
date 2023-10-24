use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use serde::*;

use crate::{states::DialogState, views::icons::ok_button_icon};

#[derive(Props, PartialEq, Eq)]
pub struct ShowSecretProps {
    pub secret: String,
}

pub fn show_secret<'s>(cx: Scope<'s, ShowSecretProps>) -> Element {
    let value_state = use_state(cx, || "".to_string());

    if value_state.get().is_empty() {
        let secret_id = cx.props.secret.to_string();
        let value_state_owned = value_state.to_owned();
        cx.spawn(async move {
            let result = load_secret(secret_id).await.unwrap();
            value_state_owned.set(result.value);
        })
    }

    render! {
        div { class: "modal-content",
            div { class: "form-floating mb-3",
                input { class: "form-control", readonly: true, value: "{value_state.get()}" }
                label { "Secret value" }
            }
        }
        div { class: "modal-footer",
            div { class: "btn-group",
                button {
                    class: "btn btn-primary",
                    onclick: move |_| {
                        use_shared_state::<DialogState>(cx).unwrap().write().hide_dialog();
                    },
                    ok_button_icon {}
                    "Close"
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SecretValueApiModel {
    pub value: String,
}

#[server]
async fn load_secret<'s>(secret_id: String) -> Result<SecretValueApiModel, ServerFnError> {
    let response = crate::grpc_client::SecretsGrpcClient::get_secret(secret_id)
        .await
        .unwrap();

    let result = SecretValueApiModel {
        value: response.value,
    };

    Ok(result)
}
