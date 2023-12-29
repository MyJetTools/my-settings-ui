use std::rc::Rc;

use dioxus::{html::GlobalAttributes, prelude::*};
use dioxus_fullstack::prelude::*;

use crate::{
    states::{CloudFlareRecordsState, DialogState},
    views::{icons::*, *},
};

#[derive(Props, PartialEq, Eq)]
pub struct EditCfARecordProps {
    pub domain: Rc<String>,
    pub proxied: bool,
}

#[component]
pub fn EditCfRecord(
    cx: Scope,
    domain: Rc<String>,
    proxied: bool,
    lb_ip: String,
    cf_record_id: String,
) -> Element {
    let ip_state = use_state(cx, || lb_ip.to_string());

    let ip_state_value = ip_state.get().as_str();

    let req_is_being_processed = use_state(cx, || false);

    let save_icon = if *req_is_being_processed.get() {
        rsx! { wait_button_icon {} }
    } else {
        rsx! { ok_button_icon {} }
    };

    let cf_rec_id_content = if cf_record_id.len() > 0 {
        rsx! { div { "{cf_record_id}" } }
    } else {
        rsx! { div {} }
    };

    render! {
        cf_rec_id_content,
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
                        ip_state.set(cx.value.to_string());
                    }
                }
                label { "Ip" }
            }
        }

        div { class: "modal-footer",
            div { class: "btn-group",
                button {
                    class: "btn btn-primary",
                    disabled: *req_is_being_processed.get(),
                    onclick: move |_| {
                        let dialog_state = use_shared_state::<DialogState>(cx).unwrap().to_owned();
                        let cf_records_state = use_shared_state::<CloudFlareRecordsState>(cx)
                            .unwrap()
                            .to_owned();
                        let ip = lb_ip.to_string();
                        let id = cf_record_id.to_string();
                        let proxied = *proxied;
                        let domain = domain.to_string();
                        req_is_being_processed.set(true);
                        cx.spawn(async move {
                            let _ = set_dns_record(domain, id, ip, proxied).await;
                            cf_records_state.write().reset_value();
                            dialog_state.write().hide_dialog();
                        });
                    },
                    ok_button_icon {}
                    "Save"
                }
                button {
                    class: "btn btn-outline-dark",
                    onclick: move |_| {
                        let dialog_state = use_shared_state::<DialogState>(cx).unwrap();
                        dialog_state.write().hide_dialog();
                    },
                    save_icon,
                    "Cancel"
                }
            }
        }
    }
}

#[server]
async fn set_dns_record(
    domain: String,
    id: String,
    ip: String,
    proxied: bool,
) -> Result<(), ServerFnError> {
    let domain_zone = crate::utils::extract_domain_name(&domain);

    if id.len() > 0 {
        crate::cf_http_client::delete_dns_record(domain_zone, id.as_str()).await;
    }

    crate::cf_http_client::create_a_record(domain, proxied, ip).await;

    Ok(())
}
