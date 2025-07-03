#![allow(non_snake_case)]

use std::rc::Rc;

use dioxus::prelude::*;

mod api;
mod dialogs;
mod icons;
mod models;
mod states;
mod storage;
mod ui_utils;
mod utils;
mod views;
use dioxus_utils::DataState;
use serde::*;

#[cfg(feature = "server")]
mod server;

use crate::{icons::*, states::*};

#[derive(Routable, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AppRoute {
    #[route("/")]
    Home,
    #[route("/templates")]
    Templates,
    #[route("/secrets")]
    Secrets,
}

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(server_only!(ServeConfig::builder().incremental(
            IncrementalRendererConfig::default()
                .invalidate_after(std::time::Duration::from_secs(120)),
        )))
        .launch(|| {
            rsx! {
                Router::<AppRoute> {}
            }
        })
}

#[component]
fn Home() -> Element {
    use_context_provider(|| Signal::new(MainState::new(LocationState::None)));
    rsx! {
        MyLayout {}
    }
}

#[component]
fn Templates() -> Element {
    use_context_provider(|| Signal::new(MainState::new(LocationState::Templates)));
    rsx! {
        MyLayout {}
    }
}

#[component]
fn Secrets() -> Element {
    use_context_provider(|| Signal::new(MainState::new(LocationState::Secrets)));
    rsx! {
        MyLayout {}
    }
}

#[component]
fn MyLayout() -> Element {
    use crate::dialogs::*;
    use crate::views::*;

    use_context_provider(|| Signal::new(DialogState::None));
    use_context_provider(|| Signal::new(FilterSecret::new()));
    use_context_provider(|| Signal::new(FilterTemplate::new()));

    let mut main_state = consume_context::<Signal<MainState>>();

    let main_state_read_access = main_state.read();

    match main_state_read_access.envs.as_ref() {
        DataState::None => {
            spawn(async move {
                match get_envs().await {
                    Ok(resp) => {
                        let mut write_access = main_state.write();

                        if resp.envs.is_empty() {
                            write_access.envs = DataState::Error("Unauthorized access".to_string());
                            return;
                        }

                        write_access.set_envs(resp.envs.into_iter().map(Rc::new).collect());

                        write_access.user = resp.name;
                        write_access.prompt_ssh_key = Some(resp.prompt_ssh_pass_key);
                    }
                    Err(err) => {
                        main_state.write().envs = DataState::Error(err.to_string());
                    }
                }
            });
            return rsx! {
                div { "Loading envs..." }
            };
        }

        DataState::Loading => {
            return {
                rsx! {
                    div { "Loading envs..." }
                    LoadingIcon {}
                }
            };
        }
        DataState::Loaded(_) => {}

        DataState::Error(err) => {
            return {
                rsx! {
                    div { {err.as_str()} }
                }
            }
        }
    }

    if main_state_read_access.prompt_ssh_key.unwrap_or(false) {
        return rsx! {
            PromptSshPassKey {}
        };
    }

    rsx! {
        div { id: "layout",
            div { id: "left-panel", LeftPanel {} }
            div { id: "right-panel", RightPanel {} }
            RenderDialog {}
            RenderToast {}
        }
    }
}

#[component]
fn RenderToast() -> Element {
    rsx! {
        div {
            id: "liveToast",
            style: "position: absolute !important;margin-bottom: 10px !important;margin-left: 10px !important; z-index: 5000;",
            class: "toast bottom-0 start-0 text-bg-danger",
            role: "alert",
            aria_live: "assertive",
            aria_atomic: "true",
            div { class: "d-flex",
                div { id: "toast-message", class: "toast-body" }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvsHttpResponse {
    pub name: String,
    pub envs: Vec<String>,
    prompt_ssh_pass_key: bool,
}

#[server]
pub async fn get_envs() -> Result<EnvsHttpResponse, ServerFnError> {
    let server_context = server_context();

    let user_id = {
        let req = server_context.request_parts();

        if let Some(user) = req.headers.get("x-ssl-user") {
            user.to_str().unwrap().to_string()
        } else {
            "".to_string()
        }
    };

    println!("Sending envs for user: [{}]", user_id);

    let (envs, prompt_ssh_pass_key) = crate::server::APP_CTX.get_envs(&user_id).await;

    Ok(EnvsHttpResponse {
        name: user_id,
        envs,
        prompt_ssh_pass_key,
    })
}
