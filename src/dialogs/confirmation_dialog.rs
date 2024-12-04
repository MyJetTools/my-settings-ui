use dioxus::prelude::*;

use super::*;

#[component]
pub fn ConfirmationDialog(content: String, on_ok: EventHandler<()>) -> Element {
    rsx! {
        DialogTemplate {
            header: "Confirmation".to_string(),
            content: rsx! {
                {content}
            },
            ok_button: rsx! {
                button {
                    class: "btn btn-success",
                    onclick: move |_| {
                        consume_context::<Signal<DialogState>>().set(DialogState::None);
                        on_ok.call(());
                    },
                    "Confirm"
                }
            }
        }
    }
}
