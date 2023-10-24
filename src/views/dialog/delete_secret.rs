use crate::{
    states::{DialogState, MainState},
    views::icons::*,
};
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

#[derive(Props, PartialEq, Eq)]
pub struct DeleteSecretProps {
    pub secret: String,
}
pub fn delete_secret_dialog<'s>(cx: Scope<'s, DeleteSecretProps>) -> Element {
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
                        let main_state = use_shared_state::<MainState>(cx).unwrap().to_owned();
                        let dialog_state: UseSharedState<DialogState> = use_shared_state::<DialogState>(cx)
                            .unwrap()
                            .to_owned();
                        let secret_id = cx.props.secret.to_string();
                        cx.spawn(async move {
                            delete_secret(secret_id).await.unwrap();
                            dialog_state.write().hide_dialog();
                            main_state.write().set_secrets(None);
                        });
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

#[server]
async fn delete_secret(secret_id: String) -> Result<(), ServerFnError> {
    crate::grpc_client::SecretsGrpcClient::delete_secret(secret_id)
        .await
        .unwrap();

    Ok(())
    /*
    let name = cx.props.secret.clone();

    let main_state = use_shared_state::<MainState>(cx).unwrap().to_owned();
    let dialog_state: UseSharedState<DialogState> =
        use_shared_state::<DialogState>(cx).unwrap().to_owned();

    cx.spawn(async move {


        dialog_state.write().hide_dialog();
        main_state.write().set_secrets(None);
    })
     */
}
