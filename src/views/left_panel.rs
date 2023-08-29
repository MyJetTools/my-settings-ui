use dioxus::prelude::*;

use crate::states::MainState;

const ACTIVE_CLASS: &str = "menu-active";

pub fn left_panel(cx: Scope) -> Element {
    let main_state = use_shared_state::<MainState>(cx).unwrap();

    let mut secrets_active = "";
    let mut templates_active = "";

    match &*main_state.read() {
        MainState::Nothing => {}
        MainState::Templates(_) => {
            templates_active = ACTIVE_CLASS;
        }
        MainState::Secrets(_) => {
            secrets_active = ACTIVE_CLASS;
        }
    }

    render! {
        h1 { "Setting" }
        h4 { id: "env-type", "demo" }

        div { id: "menu",
            div {
                class: "menu-item {secrets_active}",
                onclick: move |_| {
                    if !main_state.read().is_secrets() {
                        main_state.write().set_secrets(None);
                    }
                },
                "Secrets"
            }
            div {
                class: "menu-item {templates_active}",
                onclick: move |_| {
                    if !main_state.read().is_templates() {
                        main_state.write().set_templates(None);
                    }
                },
                "Templates"
            }
        }
    }
}
