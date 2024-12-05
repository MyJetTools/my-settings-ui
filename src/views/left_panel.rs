use super::*;
use crate::{states::*, AppRoute};
use dioxus::prelude::*;
use dioxus_router::prelude::Link;

const ACTIVE_CLASS: &str = "menu-active";

#[component]
pub fn LeftPanel() -> Element {
    let mut main_state = consume_context::<Signal<MainState>>();

    let main_state_read_access = main_state.read();

    let current_location = main_state_read_access.location.clone();

    let mut secrets_active = "";
    let mut templates_active = "";

    match current_location {
        LocationState::None => {}
        LocationState::Templates => {
            templates_active = ACTIVE_CLASS;
        }
        LocationState::Secrets => {
            secrets_active = ACTIVE_CLASS;
        }
    }

    rsx! {
        EnvsSelector {}
        h1 { "Settings" }

        div { {main_state_read_access.user.as_str()} }

        div { id: "menu",
            div { class: "menu-item {secrets_active}",
                Link {
                    to: AppRoute::Secrets,
                    onclick: move |_| {
                        if !current_location.is_secrets() {
                            main_state.write().set_location(LocationState::Secrets);
                        }
                    },
                    "Secrets"
                }
            }
            div { class: "menu-item {templates_active}",
                Link {
                    to: AppRoute::Templates,
                    onclick: move |_| {
                        if !current_location.is_templates() {
                            main_state.write().set_location(LocationState::Templates);
                        }
                    },
                    "Templates"
                }
            }
        }
    }
}
