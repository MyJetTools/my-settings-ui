use std::sync::Arc;

use app_ctx::AppCtx;
use dioxus::prelude::*;
use dioxus_liveview::LiveViewPool;
use salvo::prelude::*;

mod api_client;
mod app_ctx;
mod http_server;
mod settings;
mod states;
mod static_resources;
mod views;

use settings::SettingsReader;
use views::*;

use crate::states::*;

lazy_static::lazy_static! {
    pub static ref APP_CTX: AppCtx = {
        AppCtx::new()
    };
}

#[tokio::main]
async fn main() {
    let settings_reader = crate::settings::SettingsReader::new(".my-settings-ui").await;
    let settings_reader: Arc<SettingsReader> = Arc::new(settings_reader);
    APP_CTX.inject_settings(settings_reader).await;

    let acceptor = TcpListener::new("0.0.0.0:9001").bind().await;
    let view = LiveViewPool::new();

    let router = Router::new()
        .hoop(affix::inject(Arc::new(view)))
        .get(http_server::index)
        .push(Router::with_path("ws").get(http_server::connect))
        .push(Router::with_path("img/<**path>").get(StaticDir::new("./files/img")));

    Server::new(acceptor).serve(router).await;
}

fn app(cx: Scope) -> Element {
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
