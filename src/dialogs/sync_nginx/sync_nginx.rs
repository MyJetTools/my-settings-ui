use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use std::rc::Rc;

use crate::{
    states::{DialogState, MainState},
    views::{icons::*, NginxConfigHttpModel},
};

#[component]
pub fn SyncNginxConfiguration(cx: Scope, domain: Rc<String>, config: Rc<String>) -> Element {
    let model: NginxConfigHttpModel = serde_json::from_str(&config).unwrap();

    let model_to_save = model.clone();

    let ca = if let Some(ca) = model.ca {
        ca
    } else {
        "".to_string()
    };

    let template = if let Some(template) = model.template {
        template
    } else {
        "".to_string()
    };

    let items = model.routes.into_iter().map(|itm| {
        let path = itm.0;
        let proxy_to = itm.1.proxy_to;
        let template = itm.1.template.unwrap_or("".to_string());
        rsx! {
            tr {
                td { "{path}" }
                td { "{proxy_to}" }
                td { "{template}" }
            }
        }
    });

    render! {

        div {
            div { class: "form-floating mb-3",
                input { class: "form-control", disabled: true, value: "{domain.as_str()}" }
                label { "Domain" }
            }
            div { class: "form-floating mb-3",
                input { class: "form-control", disabled: true, value: "{ca}" }
                label { "CA" }
            }
            div { class: "form-floating mb-3",
                input { class: "form-control", disabled: true, value: "{template}" }
                label { "Template" }
            }

            table { class: "table table-striped table-sm",
                tr {
                    th { "Path" }
                    th { "Proxy pass" }
                    th { "Template" }
                }
                items
            }
        }
        div { class: "modal-footer",
            div { class: "btn-group",
                button {
                    class: "btn btn-primary",
                    onclick: move |_| {
                        let dialog_state = use_shared_state::<DialogState>(cx).unwrap().to_owned();
                        let main_state = use_shared_state::<MainState>(cx).unwrap().to_owned();
                        let domain = domain.to_string();
                        let model = model_to_save.clone();
                        cx.spawn(async move {
                            sync_with_nginx(domain, model).await.unwrap();
                            main_state.write().set_domains(None);
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
                    "Cancel"
                }
            }
        }
    }
}

#[server]
pub async fn sync_with_nginx(
    domain: String,
    model: NginxConfigHttpModel,
) -> Result<(), ServerFnError> {
    crate::nginx_http_client::save_nginx_configuration(domain, model).await;
    Ok(())
}
