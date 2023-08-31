use std::sync::Arc;

use dioxus_liveview::LiveViewPool;

use salvo::http::HeaderValue;
use salvo::prelude::*;

#[handler]
pub fn index(req: &Request, res: &mut Response) {
    res.headers.append(
        "Content-Type",
        HeaderValue::from_bytes("text/html; charset=uft-8".as_bytes()).unwrap(),
    );

    let host = match req.headers().get("Host") {
        Some(value) => value.to_str().unwrap(),
        None => "localhost:9001",
    };

    let scheme = match req.headers().get("X-Forwarded-Proto") {
        Some(value) => {
            if value.to_str().unwrap().starts_with("https") {
                "wss"
            } else {
                "ws"
            }
        }
        None => {
            if host.starts_with("localhost") || host.starts_with("127.0.0") {
                "ws"
            } else {
                "wss"
            }
        }
    };

    let ws = format!("{}://{}", scheme, host);

    res.write_body(super::static_resources::get_html(ws.as_str()).into_bytes())
        .unwrap();
}

#[handler]
pub async fn connect(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<(), StatusError> {
    let view = depot.obtain::<Arc<LiveViewPool>>().unwrap().clone();

    WebSocketUpgrade::new()
        .upgrade(req, res, |ws| async move {
            _ = view
                .launch(dioxus_liveview::salvo_socket(ws), crate::app)
                .await;
        })
        .await
}
