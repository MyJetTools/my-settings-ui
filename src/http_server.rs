use std::sync::Arc;

use dioxus_liveview::LiveViewPool;

use rust_extensions::StrOrString;
use salvo::http::HeaderValue;
use salvo::prelude::*;

#[handler]
pub fn index(res: &mut Response) {
    res.headers.append(
        "Content-Type",
        HeaderValue::from_bytes("text/html; charset=uft-8".as_bytes()).unwrap(),
    );

    let ws: StrOrString<'static> = match std::env::var("WS_HOST") {
        Ok(ws) => ws.into(),
        Err(_) => "ws://localhost:9001".into(),
    };

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
