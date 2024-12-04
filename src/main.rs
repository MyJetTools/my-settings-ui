#![allow(non_snake_case)]

use std::rc::Rc;

use dioxus::prelude::*;

mod dialogs;
mod states;
mod storage;
mod ui_utils;
mod utils;
mod views;
use dioxus_utils::DataState;
use icons::LoadingIcon;
use serde::*;
use views::*;
#[cfg(feature = "server")]
mod server;

use crate::states::*;

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
    let cfg = dioxus::fullstack::Config::new();

    #[cfg(feature = "server")]
    let cfg = cfg.addr(([0, 0, 0, 0], 9001));

    LaunchBuilder::fullstack().with_cfg(cfg).launch(|| {
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

    use_context_provider(|| Signal::new(DialogState::None));
    use_context_provider(|| Signal::new(FilterSecret::new()));
    use_context_provider(|| Signal::new(FilterTemplate::new()));

    let mut main_state = consume_context::<Signal<MainState>>();

    let main_state_read_access = main_state.read();

    match main_state_read_access.envs.as_ref() {
        DataState::None => {
            spawn(async move {
                match get_envs().await {
                    Ok(envs) => {
                        main_state.write().envs =
                            DataState::Loaded(envs.into_iter().map(Rc::new).collect());
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

#[server]
pub async fn get_envs() -> Result<Vec<String>, ServerFnError> {
    let result = crate::server::APP_CTX.get_envs().await;

    Ok(result)
}
