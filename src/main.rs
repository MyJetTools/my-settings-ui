#![allow(non_snake_case)]

use std::rc::Rc;

use dioxus::prelude::*;

mod api;
mod components;
mod dialogs;
mod icons;
mod models;
mod states;
mod storage;
mod ui_utils;
mod utils;
mod views;
use dioxus_utils::*;
use serde::*;

#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
use dioxus::server::*;

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

    let ms = consume_context::<Signal<MainState>>();

    let ms_ra = ms.read();

    if let Err(err) = init_envs(ms, &ms_ra) {
        return err;
    }

    if ms_ra.prompt_ssh_key.unwrap_or(false) {
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

fn init_envs(mut ms: Signal<MainState>, ms_ra: &MainState) -> Result<(), Element> {
    match ms_ra.envs.as_ref() {
        RenderState::None => {
            spawn(async move {
                let envs_resp = match crate::api::envs::get_envs().await {
                    Ok(resp) => {
                        if resp.envs.is_empty() {
                            ms.write().envs.set_error("Unauthorized access".to_string());
                            return;
                        }
                        resp
                    }
                    Err(err) => {
                        ms.write().envs.set_error(err.to_string());
                        return;
                    }
                };

                let envs: Vec<_> = envs_resp.envs.into_iter().map(Rc::new).collect();
                let selected_env = envs.first().unwrap().clone();

                let templates =
                    match crate::api::templates::get_templates(selected_env.to_string()).await {
                        Ok(templates) => templates,
                        Err(err) => {
                            ms.write().envs.set_error(err.to_string());
                            return;
                        }
                    };

                let mut write_access = ms.write();
                write_access.set_envs(envs, selected_env);
                write_access.user = envs_resp.name;
                write_access.prompt_ssh_key = Some(envs_resp.prompt_ssh_pass_key);
                write_access.set_templates_as_loaded(templates);
            });
            let result = rsx! {
                div { "Loading envs..." }
            };

            return Err(result);
        }

        RenderState::Loading => {
            let loading_icon = crate::icons::loading_icon();
            let result = {
                rsx! {
                    div { "Loading envs..." }
                    {loading_icon}
                }
            };

            return Err(result);
        }
        RenderState::Loaded(_) => return Ok(()),

        RenderState::Error(err) => {
            let result = {
                rsx! {
                    div { {err.as_str()} }
                }
            };
            return Err(result);
        }
    }
}
