use std::{collections::BTreeMap, rc::Rc};

use dioxus::{html::GlobalAttributes, prelude::*};
use dioxus_fullstack::prelude::*;
use serde::*;

use crate::{
    states::*,
    views::{
        dialog::edit_domain_product::state::EditDomainProductState, icons::*, NginxConfigHttpModel,
        NginxRouteHttpModel,
    },
};

#[derive(Debug, Clone)]
pub struct NginxSetupState {
    pub ca: String,
    pub template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NginxRoute {
    pub proxy_to: String,
    pub template: String,
}

#[component]
pub fn EditDomainProduct(
    cx: Scope,
    add: bool,
    name: String,
    is_cloud_flare_proxy_pass: bool,
    nginx_config: String,
) -> Element {
    let widget_state = use_ref(cx, || {
        EditDomainProductState::new(*add, *is_cloud_flare_proxy_pass, name.as_str())
    });

    let (nginx_setup_state, entered_routes) = if nginx_config.is_empty() {
        let nginx_setup_state = use_ref(cx, || NginxSetupState {
            ca: "".to_string(),
            template: "".to_string(),
        });

        let entered_routes: &UseRef<BTreeMap<String, NginxRoute>> = use_ref(cx, || BTreeMap::new());

        (nginx_setup_state, entered_routes)
    } else {
        let data: NginxConfigHttpModel = serde_json::from_str(nginx_config.as_str()).unwrap();
        let nginx_setup_state = use_ref(cx, || NginxSetupState {
            ca: data.ca.unwrap_or("".to_string()),
            template: data.template.unwrap_or("".to_string()),
        });

        let entered_routes: &UseRef<BTreeMap<String, NginxRoute>> = use_ref(cx, || {
            let mut result = BTreeMap::new();

            for (path, route) in data.routes {
                result.insert(
                    path,
                    NginxRoute {
                        proxy_to: route.proxy_to,
                        template: route.template.unwrap_or("".to_string()),
                    },
                );
            }
            result
        });

        (nginx_setup_state, entered_routes)
    };

    let nginx_templates_state: &UseState<Option<NginxData>> = use_state(cx, || None);

    let edit_path = use_ref(cx, || "".to_string());
    let edit_route = use_ref(cx, || NginxRoute {
        proxy_to: "".to_string(),
        template: "".to_string(),
    });

    if nginx_templates_state.get().is_none() {
        let nginx_templates_state = nginx_templates_state.clone();
        cx.spawn(async move {
            let templates = load_nginx_templates().await.unwrap();
            nginx_templates_state.set(Some(templates));
        });
        return render!( h3 { "Loading Nginx Templates..." } );
    }

    let nginx_data = nginx_templates_state.get().as_ref().unwrap();

    let mut template_options = nginx_data
        .templates
        .iter()
        .map(|key| {
            render! { option { value: "{key}", "{key}" } }
        })
        .collect::<Vec<Element>>();

    template_options.insert(0, render! { option { value: "", "" } });

    let mut ca_options = nginx_data
        .ca
        .iter()
        .map(|key| {
            render! { option { value: "{key}", "{key}" } }
        })
        .collect::<Vec<Element>>();

    ca_options.insert(0, render! { option { value: "", "" } });

    let save_button_is_disabled = !widget_state.read().can_be_saved();

    let cloud_flare_value = if widget_state.read().is_cloud_flare_proxy {
        "true"
    } else {
        "false"
    };

    let nginx_setup_state_value = nginx_setup_state.read().clone();

    let entered_routes_table = {
        let values = entered_routes.read().clone();
        values.into_iter().map(|itm| {
        let key = Rc::new(itm.0);
        let key_delete = key.clone();
        rsx! {
            tr {
                td { "{key}" }
                td { "{itm.1.proxy_to}" }
                td { "{itm.1.template}" }
                td {
                    button {
                        class: "btn btn-outline-dark btn-sm",
                        onclick: move |_| {
                            let item = entered_routes.read().get(key.as_str()).unwrap().clone();
                            let mut edit_path_value = edit_path.write();
                            *edit_path_value = key.as_str().to_string();
                            let mut edit_route_value = edit_route.write();
                            edit_route_value.proxy_to = item.proxy_to.clone();
                            edit_route_value.template = item.template.clone();
                        },
                        "-->"
                    }

                    button {
                        class: "btn btn-outline-dark btn-sm",
                        onclick: move |_| {
                            {
                                let item = entered_routes.read().get(key_delete.as_str()).unwrap().clone();
                                let mut edit_path_value = edit_path.write();
                                *edit_path_value = key_delete.as_str().to_string();
                                let mut edit_route_value = edit_route.write();
                                edit_route_value.proxy_to = item.proxy_to.clone();
                                edit_route_value.template = item.template.clone();
                            }
                            let mut write_access = entered_routes.write();
                            write_access.remove(key_delete.as_str());
                        },
                        delete_icon {}
                    }
                }
            }
        }
    })
    };

    let edit_path_value = edit_path.read();

    let edit_route_value = edit_route.read();

    render! {
        table { style: "width: 100%",
            tr {
                td { style: "width:70%;vertical-align: top; padding-right: 5px;",
                    div { class: "modal-content",
                        div { class: "form-floating mb-3",
                            input {
                                class: "form-control",
                                value: "{widget_state.read().get_product_name()}",
                                oninput: move |cx| {
                                    let mut widget_state = widget_state.write();
                                    widget_state.set_product_name(cx.value.as_str());
                                }
                            }
                            label { "Product name" }
                        }
                        div { class: "form-floating mb-3",
                            select {
                                class: "form-control",
                                onchange: move |cx| {
                                    let mut widget_state = widget_state.write();
                                    widget_state.is_cloud_flare_proxy = cx.value == "true";
                                },
                                value: "{cloud_flare_value}",
                                option { value: "false", "DNS Only" }
                                option { value: "true", "Proxy pass" }
                            }
                            label { "Cloudflare configuration" }
                        }
                    }
                    hr {
                    }
                    label { "Nginx configuration" }

                    div { class: "form-floating mb-3",
                        select {
                            class: "form-control",
                            value: "{nginx_setup_state_value.template}",
                            onchange: move |cx| {
                                let mut nginx_setup_state = nginx_setup_state.write();
                                nginx_setup_state.template = cx.value.to_string();
                            },
                            template_options.clone().into_iter()
                        }
                        label { "Template" }
                    }

                    div { class: "form-floating mb-3",
                        select {
                            class: "form-control",
                            value: "{nginx_setup_state_value.ca}",
                            onchange: move |cx| {
                                let mut nginx_setup_state = nginx_setup_state.write();
                                nginx_setup_state.ca = cx.value.to_string();
                            },
                            ca_options.into_iter()
                        }
                        label { "Protected with CA" }
                    }

                    table { class: "table table-striped table-sm",
                        tr {
                            th { "Path" }
                            th { "Proxy to" }
                            th { "Template" }
                            th {}
                        }
                        { entered_routes_table
                            }
                    }
                }
                td { style: "vertical-align: top; padding-left: 5px; border-left: 1px solid lightgray;",

                    div { class: "form-floating mb-3",
                        input {
                            class: "form-control",
                            value: "{edit_path_value.as_str()}",
                            oninput: move |cx| {
                                let mut value = edit_path.write();
                                *value = cx.value.to_string();
                            }
                        }
                        label { "Path" }
                    }

                    div { class: "form-floating mb-3",
                        input {
                            class: "form-control",
                            value: "{edit_route_value.proxy_to.as_str()}",
                            oninput: move |cx| {
                                let mut edit_route = edit_route.write();
                                edit_route.proxy_to = cx.value.to_string();
                            }
                        }
                        label { "Proxy to" }
                    }

                    div { class: "form-floating mb-3",
                        select {
                            class: "form-control",
                            value: "{edit_route_value.template.as_str()}",
                            oninput: move |cx| {
                                let mut edit_route = edit_route.write();
                                edit_route.template = cx.value.to_string();
                            },
                            template_options.into_iter()
                        }
                        label { "Template" }
                    }

                    button {

                        class: "btn btn-outline-dark",

                        onclick: move |_| {
                            let name = edit_path.read().to_string();
                            let value = edit_route.read().clone();
                            entered_routes.write().insert(name, value);
                            widget_state.write().set_nginx_config_has_changes();
                        },
                        "<-- Add route"
                    }
                }
            }
        }

        div { class: "modal-footer",
            div { class: "btn-group",
                button {
                    class: "btn btn-primary",
                    disabled: save_button_is_disabled,
                    onclick: move |_| {
                        let main_state = use_shared_state::<MainState>(cx).unwrap().to_owned();
                        let dialog_state = use_shared_state::<DialogState>(cx).unwrap().to_owned();
                        let (product_name, is_cloud_flare_proxy) = {
                            let widget_state = widget_state.read();
                            (widget_state.get_product_name().to_string(), widget_state.is_cloud_flare_proxy)
                        };
                        let nginx_config = compile_nginx_config(
                            &nginx_setup_state_value,
                            &entered_routes.read(),
                        );
                        cx.spawn(async move {
                            save_domain_product(product_name, is_cloud_flare_proxy, nginx_config)
                                .await
                                .unwrap();
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
                        use_shared_state::<DialogState>(cx).unwrap().write().hide_dialog();
                    },
                    cancel_button_icon {}
                    "Cancel"
                }
            }
        }
    }
}

fn compile_nginx_config(
    nginx_setup_state: &NginxSetupState,
    entered_routes: &BTreeMap<String, NginxRoute>,
) -> Option<NginxConfigHttpModel> {
    if entered_routes.len() == 0 {
        return None;
    }

    let routes: BTreeMap<String, _> = entered_routes
        .iter()
        .map(|(key, value)| {
            let result = (
                key.to_string(),
                NginxRouteHttpModel {
                    proxy_to: value.proxy_to.clone(),
                    template: if value.template.as_str().is_empty() {
                        None
                    } else {
                        Some(value.template.clone())
                    },
                },
            );
            result
        })
        .collect();

    let result = NginxConfigHttpModel {
        ca: if nginx_setup_state.ca.as_str().is_empty() {
            None
        } else {
            Some(nginx_setup_state.ca.clone())
        },
        template: if nginx_setup_state.template.as_str().is_empty() {
            None
        } else {
            Some(nginx_setup_state.template.clone())
        },
        routes,
    };

    Some(result)
}

#[server]
async fn save_domain_product<'s>(
    product_name: String,
    is_cloud_flare_proxy: bool,
    nginx: Option<NginxConfigHttpModel>,
) -> Result<(), ServerFnError> {
    let nginx_config = if let Some(nginx) = nginx {
        let result = crate::domains_grpc::NginxConfigGrpcModel {
            protected_with_ca: nginx.ca,
            template: nginx.template,
            routes: nginx
                .routes
                .into_iter()
                .map(|route| crate::domains_grpc::NginxRouteGrpcModel {
                    path: route.0,
                    proxy_to: route.1.proxy_to,
                    template: route.1.template,
                })
                .collect(),
        };

        Some(result)
    } else {
        None
    };
    crate::grpc_client::DomainsGrpcClient::save(product_name, is_cloud_flare_proxy, nginx_config)
        .await
        .unwrap();

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NginxData {
    templates: Vec<String>,
    ca: Vec<String>,
}

#[server]
async fn load_nginx_templates<'s>() -> Result<NginxData, ServerFnError> {
    let url = crate::APP_CTX.settings.get_nginx_api();

    let fl_url = flurl::FlUrl::new(url.as_str());

    let mut response = fl_url
        .append_path_segment("api")
        .append_path_segment("nginx")
        .append_path_segment("templates")
        .append_path_segment("v1")
        .get()
        .await
        .unwrap();

    let templates: std::collections::BTreeMap<String, Vec<String>> =
        response.get_json().await.unwrap();

    let fl_url = flurl::FlUrl::new(url.as_str());
    let mut response = fl_url
        .append_path_segment("api")
        .append_path_segment("ca")
        .append_path_segment("v1")
        .append_path_segment("list")
        .get()
        .await
        .unwrap();

    let ca: Vec<String> = response.get_json().await.unwrap();

    let templates = NginxData {
        templates: templates.into_iter().map(|(key, _)| key).collect(),
        ca,
    };

    Ok(templates)
}
