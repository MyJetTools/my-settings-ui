use dioxus::prelude::*;

use crate::{states::MainState, APP_CTX};

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

    let env_name = use_state(cx, || "".to_string());

    if env_name.get() == "" {
        let env_name_own = env_name.to_owned();

        cx.spawn(async move {
            let env_name = tokio::spawn(async move {
                let reader = APP_CTX.get_settings_reader().await;
                reader.get_env_name().await
            })
            .await
            .unwrap();

            env_name_own.set(env_name);
        });
    }

    render! {
        h1 { "Settings" }
        h4 { id: "env-type", "{env_name.get()}" }

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
