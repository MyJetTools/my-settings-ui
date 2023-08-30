use dioxus::prelude::*;

use crate::{
    states::{DialogState, MainState},
    views::icons::*,
};

#[derive(Props, PartialEq, Eq)]
pub struct DeleteSecretProps {
    pub secret: String,
}
pub fn delete_secret<'s>(cx: Scope<'s, DeleteSecretProps>) -> Element {
    let content = format!("You are about to delete a secret '{}'", cx.props.secret);
    render! {
        div { class: "modal-content",
            h4 { content }
        }
        div { class: "modal-footer",
            div { class: "btn-group",
                button {
                    class: "btn btn-primary",
                    onclick: move |_| {
                        do_delete_secret(cx);
                    },
                    ok_button_icon {}
                    "Save"
                }
                button {
                    class: "btn btn-outline-dark",
                    onclick: move |_| {
                        use_shared_state::<DialogState>(cx).unwrap().write().hide_dialog();
                    },
                    cancel_button_icon {}
                    "Cancel"
                }
            }
        }
    }
}

fn do_delete_secret<'s>(cx: &'s Scoped<'s, DeleteSecretProps>) {
    let name = cx.props.secret.clone();

    let main_state = use_shared_state::<MainState>(cx).unwrap().to_owned();
    let dialog_state: UseSharedState<DialogState> =
        use_shared_state::<DialogState>(cx).unwrap().to_owned();

    cx.spawn(async move {
        crate::grpc_client::SecretsGrpcClient::delete_secret(name)
            .await
            .unwrap();

        dialog_state.write().hide_dialog();
        main_state.write().set_secrets(None);
    })
}
