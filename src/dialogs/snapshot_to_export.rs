use std::rc::Rc;

use dioxus::prelude::*;

use crate::dialogs::*;

#[component]
pub fn ShowTemplateToExport(yaml: Rc<String>) -> Element {
    rsx! {
        DialogTemplate {
            header: "Snapshot to export",
            allocate_max_space: true,
            content: rsx! {
                textarea { class: "form-control modal-content-full-screen", readonly: true, {yaml.as_str()} }
            },
        }
    }
}
