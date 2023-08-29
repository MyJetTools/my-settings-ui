use dioxus::prelude::*;

use crate::{
    states::{DialogState, DialogType},
    views::dialog::*,
};

pub fn render_dialog(cx: Scope) -> Element {
    let dialog = use_shared_state::<DialogState>(cx).unwrap();

    let dialog = dialog.read();
    let dialog = match dialog.as_ref() {
        DialogState::Hidden => None,
        DialogState::Shown {
            header,
            dialog_type,
        } => {
            let dialog_content = match dialog_type {
                DialogType::AddSecret => {
                    rsx! { h1 { "Add secret" } }
                }
                DialogType::ShowSecret(secret) => {
                    let secret = secret.clone();
                    rsx! { show_secret { secret: secret } }
                }
                DialogType::SecretUsage(secret) => {
                    let secret = secret.clone();
                    rsx! { show_secret_usage_by_template { secret: secret } }
                }

                DialogType::SecretUsageBySecret(secret) => {
                    let secret = secret.clone();
                    rsx! { show_secret_usage_by_secret { secret: secret } }
                }
            };

            rsx! {
                div { id: "dialog-pad",

                    div { class: "modal-dialog",
                        div { class: "modal-content",
                            div { class: "modal-header",
                                h5 { class: "modal-title", "{header}" }
                                button {
                                    r#type: "button",
                                    class: "btn btn-default btn-sm",
                                    onclick: move |_| {
                                        use_shared_state::<DialogState>(cx).unwrap().write().hide_dialog();
                                    },
                                    "X"
                                }
                            }
                            dialog_content
                        }
                    }
                }
            }
        }
        .into(),
    };

    render!(dialog)
}
