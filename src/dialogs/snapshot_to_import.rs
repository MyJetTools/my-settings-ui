use dioxus::prelude::*;

use crate::{dialogs::*, icons::OkButtonIcon};

#[component]
pub fn SnapshotToImport(on_ok: EventHandler<String>) -> Element {
    let mut cs = use_signal(|| SnapshotToImportState::default());

    let cs_ra = cs.read();

    rsx! {
        DialogTemplate {
            header: "Snapshot to export",
            allocate_max_space: true,
            content: rsx! {
                textarea {
                    class: "form-control modal-content-full-screen",
                    onchange: move |c| {
                        cs.write().value = c.value();
                    },
                    {cs_ra.value.as_str()}
                }
            },

            ok_button: rsx! {
                button {
                    class: "btn btn-primary",
                    disabled: cs_ra.value.is_empty(),
                    onclick: move |_| {
                        let result = cs.read().value.clone();
                        on_ok.call(result);
                        consume_context::<Signal<DialogState>>().set(DialogState::None);
                    },
                    OkButtonIcon {}
                    "Import"
                }
            },
        }
    }
}

#[derive(Default)]
pub struct SnapshotToImportState {
    pub value: String,
}
