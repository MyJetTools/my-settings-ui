#![allow(non_snake_case)]

use dioxus::prelude::*;

mod dialogs;
mod states;
mod utils;
mod views;
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
    use_context_provider(|| Signal::new(MainState::Nothing));
    rsx! {
        MyLayout {}
    }
}

#[component]
fn Templates() -> Element {
    use_context_provider(|| Signal::new(MainState::Templates(None)));
    rsx! {
        MyLayout {}
    }
}

#[component]
fn Secrets() -> Element {
    use_context_provider(|| Signal::new(MainState::Secrets(None)));
    rsx! {
        MyLayout {}
    }
}

/*
fn Templates(cx: Scope) -> Element {
    use_shared_state_provider(cx, || MainState::Templates(None));
    render! {
        my_layout {}
    }
}



fn Domains(cx: Scope) -> Element {
    use_shared_state_provider(cx, || MainState::Domains(None));
    render! {
        my_layout {}
    }
}
*/

#[component]
fn MyLayout() -> Element {
    use crate::dialogs::*;

    use_context_provider(|| Signal::new(DialogState::None));
    use_context_provider(|| Signal::new(FilterSecret::new()));
    use_context_provider(|| Signal::new(FilterTemplate::new()));

    rsx! {
        div { id: "layout",
            div { id: "left-panel", LeftPanel {} }
            div { id: "right-panel", RightPanel {} }
            RenderDialog {}
        }
    }
}
