use crate::{states::MainState, AppRoute};
use dioxus::prelude::*;
use dioxus_router::prelude::Link;

const ACTIVE_CLASS: &str = "menu-active";

#[component]
pub fn LeftPanel() -> Element {
    let mut main_state = consume_context::<Signal<MainState>>();

    let main_state_read_access = main_state.read();

    let mut secrets_active = "";
    let mut templates_active = "";

    match &*main_state_read_access {
        MainState::Nothing => {}
        MainState::Templates(_) => {
            templates_active = ACTIVE_CLASS;
        }
        MainState::Secrets(_) => {
            secrets_active = ACTIVE_CLASS;
        }
    }

    rsx! {
        h1 { "Settings" }

        div { id: "menu",
            div { class: "menu-item {secrets_active}",
                Link {
                    to: AppRoute::Secrets,
                    onclick: move |_| {
                        if !main_state.read().is_secrets() {
                            main_state.write().set_secrets(None);
                        }
                    },
                    "Secrets"
                }
            }
            div { class: "menu-item {templates_active}",
                Link {
                    to: AppRoute::Templates,
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
}
