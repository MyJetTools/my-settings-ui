use crate::{router::AppRoute, states::MainState};
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use dioxus_router::prelude::Link;

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

    let env_name_state: &UseState<Option<String>> = use_state(cx, || None);

    let env_name = match env_name_state.get() {
        Some(value) => value,
        None => {
            let env_name_state = env_name_state.to_owned();
            cx.spawn(async move {
                let env_name = get_env_name().await.unwrap();
                env_name_state.set(Some(env_name));
            });
            "???"
        }
    };

    render! {
        h1 { "Settings" }
        h4 { id: "env-type", "{env_name}" }

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

#[server]
async fn get_env_name() -> Result<String, ServerFnError> {
    let response = crate::grpc_client::TemplatesGrpcClient::get_env_name()
        .await
        .unwrap();

    Ok(response)
}
