use std::net::SocketAddr;
use std::sync::Arc;

use dioxus_liveview::LiveViewPool;

use salvo::http::HeaderValue;
use salvo::prelude::*;

#[handler]
pub fn index(res: &mut Response) {
    let addr: SocketAddr = ([127, 0, 0, 1], 9001).into();
    res.headers.append(
        "Content-Type",
        HeaderValue::from_bytes("text/html; charset=uft-8".as_bytes()).unwrap(),
    );

    res.write_body(super::static_resources::get_html(addr).into_bytes())
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
