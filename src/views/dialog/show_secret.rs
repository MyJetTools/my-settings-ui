use dioxus::prelude::*;

use crate::{states::DialogState, views::icons::ok_button_icon};

#[derive(Props, PartialEq, Eq)]
pub struct ShowSecretProps {
    pub secret: String,
}

pub fn show_secret<'s>(cx: Scope<'s, ShowSecretProps>) -> Element {
    let value = use_state(cx, || "".to_string());

    if value.get().is_empty() {
        load_secret(&cx, cx.props.secret.to_string(), &value);
    }

    render! {
        div { class: "modal-content",
            div { class: "form-floating mb-3",
                input { class: "form-control", readonly: true, value: "{value.get()}" }
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

fn load_secret<'s>(cx: &Scope<'s, ShowSecretProps>, secret_id: String, state: &UseState<String>) {
    let state = state.to_owned();

    cx.spawn(async move {
        let response = crate::grpc_client::SecretsGrpcClient::get_secret(secret_id)
            .await
            .unwrap();

        state.set(response.value)
    });
}
