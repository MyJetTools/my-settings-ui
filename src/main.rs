#![allow(non_snake_case)]

#[cfg(feature = "ssr")]
use app_ctx::AppCtx;
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use router::AppRoute;
#[cfg(feature = "ssr")]
mod app_ctx;
#[cfg(feature = "ssr")]
mod grpc_client;

mod router;
#[cfg(feature = "ssr")]
mod settings;
mod states;
mod utils;
mod views;
use views::*;

use crate::states::*;
#[cfg(feature = "ssr")]
lazy_static::lazy_static! {
    pub static ref APP_CTX: AppCtx = {
        AppCtx::new()
    };
}

#[cfg(feature = "ssr")]
pub mod templates_grpc {
    tonic::include_proto!("templates");
}

#[cfg(feature = "ssr")]
pub mod secrets_grpc {
    tonic::include_proto!("secrets");
}

fn main() {
    let config = LaunchBuilder::<FullstackRouterConfig<AppRoute>>::router();

    #[cfg(feature = "ssr")]
    let config = config.addr(std::net::SocketAddr::from(([0, 0, 0, 0], 8080)));

    config.launch();
}

fn Home(cx: Scope) -> Element {
    render! { my_layout {} }
}

fn my_layout(cx: Scope) -> Element {
    use_shared_state_provider(cx, || MainState::Nothing);
    use_shared_state_provider(cx, || DialogState::Hidden);

    render! {
        div { id: "layout",
            div { id: "left-panel", left_panel {} }
            div { id: "right-panel", right_panel {} }
            dialog::render_dialog {}
        }
    }
}
