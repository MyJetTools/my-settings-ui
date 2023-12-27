use std::rc::Rc;

use dioxus::{html::GlobalAttributes, prelude::*};
use dioxus_fullstack::prelude::*;

use crate::{
    states::DialogState,
    views::{icons::*, *},
};

#[derive(Props, PartialEq, Eq)]
pub struct EditCfARecordProps {
    pub domain: Rc<String>,
    pub proxied: bool,
}

#[component]
pub fn EditCfRecord(cx: Scope, domain: Rc<String>, proxied: bool) -> Element {
    let ip_state: &UseState<Option<String>> = use_state(cx, || None);

    let ip_state_value = match ip_state.get() {
        Some(value) => value.as_str(),
        None => {
            let ip_state = ip_state.to_owned();

            cx.spawn(async move {
                let ip = get_lb_ip().await.unwrap();
                ip_state.set(Some(ip));
            });

            ""
        }
    };

    render! {
        div {
            table { style: "width:100%",
                tr {
                    td { "{domain.as_str()}" }
                    td {
                        div { ProxyPassIcon { proxy_pass: *proxied, height: 16 } }
                    }
                }
            }
        }
        div {
            div { class: "form-floating mb-3",
                input {
                    class: "form-control",
                    value: "{ip_state_value}",
                    oninput: move |cx| {
                        ip_state.set(Some(cx.value.to_string()));
                    }
                }
                label { "Ip" }
            }
        }

        div { class: "modal-footer",
            div { class: "btn-group",
                button { class: "btn btn-primary", onclick: move |_| {},
                    ok_button_icon {}
                    "Save"
                }
                button {
                    class: "btn btn-outline-dark",
                    onclick: move |_| {
                        use_shared_state::<DialogState>(cx).unwrap().write().hide_dialog();
                    },
                    cancel_button_icon {}
                    "Cancel"
                }
            }
        }
    }
}

#[server]
async fn get_lb_ip<'s>() -> Result<String, ServerFnError> {
    let cloud_flare_url = crate::APP_CTX.settings.get_cloud_flare_url();

    let response = flurl::FlUrl::new(cloud_flare_url)
        .append_path_segment("api")
        .append_path_segment("InternetIp")
        .get()
        .await
        .unwrap();

    let ip = response.receive_body().await.unwrap();

    Ok(String::from_utf8(ip).unwrap())
}
