use crate::{
    states::{DialogState, MainState},
    views::icons::*,
};
use dioxus::prelude::*;

pub fn delete_secret_dialog(secret: String) -> Element {
    let content = format!("You are about to delete a secret '{}'", cx.props.secret);
    rsx! {
        div { class: "modal-content",
            h4 { content }
        }
        div { class: "modal-footer",
            div { class: "btn-group",
                button {
                    class: "btn btn-primary",
                    onclick: move |_| {
                        let secret_id = cx.props.secret.to_string();
                        spawn(async move {
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
