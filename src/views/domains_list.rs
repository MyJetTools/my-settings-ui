use std::{collections::BTreeMap, rc::Rc};

use dioxus::{html::GlobalAttributes, prelude::*};
use dioxus_fullstack::prelude::*;
use rust_extensions::StrOrString;
use serde::*;

use crate::{
    cf_http_client::CfDnsRecordRestApiModel,
    states::{CloudFlareRecordsState, DialogState, DialogType, MainState},
    views::{icons::*, *},
};

pub fn domains_list(cx: Scope) -> Element {
    let main_state = use_shared_state::<MainState>(cx).unwrap();

    let cloud_flare_domains = use_shared_state::<CloudFlareRecordsState>(cx).unwrap();

    let cf_has_values = cloud_flare_domains.read().get_value().is_some();

    let widget_state_value = main_state.read();
    let widget_state_value = widget_state_value.unwrap_as_domains();

    if widget_state_value.is_none() {
        let main_state = main_state.to_owned();
        cx.spawn(async move {
            let response = load_domains().await.unwrap();
            main_state.write().set_domains(Some(response));
        });

        return render! { h1 { "Loading..." } };
    }

    let widget_state_value = widget_state_value.unwrap();

    let mut domain_mask_read_only = false;

    let (domain_mask, add_btn) = if let Some(domain_mask) = &widget_state_value.domain_mask {
        if !cf_has_values {
            let cloud_flare_domains = cloud_flare_domains.to_owned();
            let domain = crate::utils::extract_domain_name(domain_mask).to_string();
            cx.spawn(async move {
                let response = get_cf_records(domain).await.unwrap();
                cloud_flare_domains.write().set_value(response);
            });

            return render! { h1 { "Loading Cf Domains.." } };
        }

        (
            domain_mask.as_str(),
            rsx! {
                button {
                    class: "btn btn-sm btn-primary",
                    onclick: |_| {
                        let dialog_state = use_shared_state::<DialogState>(cx).unwrap();
                        dialog_state
                            .write()
                            .show_dialog("Edit product domain".to_string(), DialogType::AddDomainProduct);
                    },
                    add_icon {}
                }
            },
        )
    } else {
        domain_mask_read_only = true;
        ("", rsx! {div{}})
    };

    let lb_ip = Rc::new(widget_state_value.lb_ip.clone());
    let products = widget_state_value.products.iter().map(|itm| {
        let name = itm.name.clone();
        let cloud_flare_proxy_pass = itm.is_cloud_flare_proxy_pass;

        let product_domain_name = Rc::new(domain_mask.replace("*", &itm.name));

        let proxy_pass = itm.is_cloud_flare_proxy_pass;

        let lb_ip = lb_ip.clone();

        let nginx_config = if let Some(nginx) = itm.nginx_config.clone() {
            Some(Rc::new(nginx))
        } else {
            None
        };

        let nginx_setup = if let Some(nginx) = itm.nginx_setup.clone() {
            Some(Rc::new(nginx))
        } else {
            None
        };

        let cx_owned = cx.to_owned();
        let nginx_config_to_render = generate_nginx_configuration(cx_owned, nginx_config.clone());

        let nginx_setup_to_render = generate_nginx_configuration(cx_owned, nginx_setup.clone());

        rsx! {
            tr { style: "border-bottom: 1px solid lightgray; text-align: left;",

                td { "{product_domain_name.as_str()}" }
                td { ProxyPassIcon { proxy_pass: proxy_pass, height: 32 } }
                td { nginx_config_to_render }
                td { nginx_setup_to_render }
                td { RenderCloudFlareStatus {
                    domain: product_domain_name.clone(),
                    ip: lb_ip.clone(),
                    proxied: proxy_pass
                } }
                td {
                    div { class: "btn-group",
                        button {
                            class: "btn btn-sm btn-primary",
                            onclick: move |_| {
                                let dialog_state = use_shared_state::<DialogState>(cx).unwrap();
                                dialog_state
                                    .write()
                                    .show_dialog(
                                        "Edit product domain".to_string(),
                                        DialogType::EditDomainProduct {
                                            name: name.clone(),
                                            cloud_flare_proxy_pass,
                                            nginx_config: nginx_config.clone(),
                                        },
                                    );
                            },
                            edit_icon {}
                        }
                    }
                }
            }
        }
    });

    let product_domains = rsx! {
        table { class: "table table-striped",
            tr { style: "border-bottom: 1px solid lightgray; text-align: left;",
                th { "Domain name" }
                th { "Cloud flare proxy pass" }
                th { "Nginx config" }
                th { "Nginx setup" }
                th { "Cloudflare status" }
                th {
                    div { add_btn }
                }
            }
            products
        }
    };

    let domain_mask_to_edit = domain_mask.to_string();

    render! {
        table { style: "width:100%",
            tr {
                td { "Domain mask is: " }
                td { input { class: "form-control", value: "{domain_mask}", readonly: domain_mask_read_only } }
                td {
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            let dialog_state = use_shared_state::<DialogState>(cx).unwrap();
                            dialog_state
                                .write()
                                .show_dialog(
                                    "Edit domain mask".to_string(),
                                    DialogType::EditDomainMask(domain_mask_to_edit.to_string()),
                                );
                        },
                        "Edit"
                    }
                }
                td { style: "width: 50%;text-align: right;", "Load balancer ip is: {lb_ip.as_ref()}" }
            }
        }

        h2 { "Product domains:" }

        product_domains
    }
}

#[component]
fn RenderCloudFlareStatus(cx: Scope, domain: Rc<String>, ip: Rc<String>, proxied: bool) -> Element {
    let domains_state = use_shared_state::<CloudFlareRecordsState>(cx).unwrap();

    let domains_state = domains_state.read();

    let domains_state = domains_state.get_value();

    if domains_state.is_none() {
        return render! { div { class: "alert alert-warning", "Loading Cloudflare info..." } };
    }

    let domains_state = domains_state.as_ref().unwrap();

    let mut cf_record_id = None;

    let result = match domains_state.get(domain.as_ref()) {
        Some(value) => {
            if value.tp != "A" {
                cf_record_id = Some(value.id.clone());
                Some(StrOrString::create_as_string(format!(
                    "Must be A-record. Found: {}",
                    value.tp
                )))
            } else if &value.content != ip.as_ref() {
                cf_record_id = Some(value.id.clone());
                Some(StrOrString::create_as_string(format!(
                    "Invalid IP: {}",
                    value.content
                )))
            } else {
                None
            }
        }
        None => Some(StrOrString::create_as_str("No Cloudflare record found")),
    };

    match result {
        Some(err) => render! {
            div {
                button {
                    class: "btn btn-danger btn-sm",
                    onclick: move |_| {
                        let dialog_state = use_shared_state::<DialogState>(cx).unwrap();
                        dialog_state
                            .write()
                            .show_dialog(
                                "Edit product domain".to_string(),
                                DialogType::EditCfDomainRecord {
                                    domain: domain.clone(),
                                    proxied: *proxied,
                                    lb_ip: ip.clone(),
                                    cf_record_id: cf_record_id.clone(),
                                },
                            );
                    },
                    "{err.as_str()}"
                }
            }
        },
        None => render! { div { class: "badge bg-success", "OK" } },
    }
}

fn generate_nginx_configuration<'s>(
    cx: &'s Scoped,
    nginx_config: Option<Rc<NginxConfigHttpModel>>,
) -> Element<'s> {
    let nginx = if let Some(nginx_config) = nginx_config {
        let ca = if let Some(ca) = nginx_config.ca.as_ref() {
            render!(
                div { style: "padding: 0;", div { class: "badge text-bg-success", "Protected with CA {ca}" } }
            )
        } else {
            render! {
                div { style: "padding: 0;", div { class: "badge text-bg-light", "No CA" } }
            }
        };

        let templates = if let Some(template) = nginx_config.template.as_ref() {
            render!(
                div { style: "padding: 0;", div { class: "badge text-bg-primary", "Template: {template}" } }
            )
        } else {
            render! {
                div { style: "padding: 0;", div { class: "badge text-bg-warning", "No global templates" } }
            }
        };

        let routes = nginx_config.routes.iter().map(|(path, route)| {
            let template = if let Some(template) = route.template.as_ref() {
                format!("+ Template: {}", template)
            } else {
                format!("")
            };

            rsx! {
                div { style: "padding: 0;",
                    div { class: "badge text-bg-dark", "'{path}' -> '{route.proxy_to}' {template}" }
                }
            }
        });

        render! { ca, templates, routes }
    } else {
        render! {
            div { style: "padding: 0;", div { class: "badge text-bg-danger", "No nginx config found" } }
        }
    };

    nginx
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainsApiModel {
    pub domain_mask: Option<String>,
    pub products: Vec<DomainProduct>,
    pub lb_ip: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainProduct {
    pub name: String,
    pub is_cloud_flare_proxy_pass: bool,
    pub nginx_config: Option<NginxConfigHttpModel>,
    pub nginx_setup: Option<NginxConfigHttpModel>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NginxConfigHttpModel {
    pub ca: Option<String>,
    pub template: Option<String>,
    pub routes: BTreeMap<String, NginxRouteHttpModel>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NginxRouteHttpModel {
    pub proxy_to: String,
    pub template: Option<String>,
}

#[server]
pub async fn load_domains() -> Result<DomainsApiModel, ServerFnError> {
    let response = crate::grpc_client::DomainsGrpcClient::get().await.unwrap();

    let lb_ip = crate::cf_http_client::get_lb_ip().await;

    let mut nginx_setup = crate::nginx_http_client::get_list_of_configurations().await;

    let domain_mask = response.domain_mask.clone();

    let result = DomainsApiModel {
        domain_mask: response.domain_mask,
        products: response
            .products
            .into_iter()
            .map(|itm| {
                let nginx_setup = if let Some(domain_mask) = domain_mask.as_ref() {
                    let domain_to_find = domain_mask.replace("*", itm.product_name.as_str());

                    find_nginx_setup(&mut nginx_setup, domain_to_find.as_str())
                } else {
                    None
                };

                DomainProduct {
                    name: itm.product_name,
                    is_cloud_flare_proxy_pass: itm.is_cloud_flare_proxy,
                    nginx_config: if let Some(nginx) = itm.nginx_config {
                        Some(nginx.into())
                    } else {
                        None
                    },
                    nginx_setup,
                }
            })
            .collect(),
        lb_ip,
    };

    Ok(result)
}

#[server]
async fn get_cf_records(domain: String) -> Result<Vec<CfDnsRecordRestApiModel>, ServerFnError> {
    let response = crate::cf_http_client::get_dns_records(domain).await;
    Ok(response)
}

#[cfg(feature = "ssr")]
impl Into<NginxConfigHttpModel> for crate::domains_grpc::NginxConfigGrpcModel {
    fn into(self) -> NginxConfigHttpModel {
        let result = NginxConfigHttpModel {
            ca: self.protected_with_ca,
            template: self.template,
            routes: self
                .routes
                .into_iter()
                .map(|route| {
                    (
                        route.path,
                        NginxRouteHttpModel {
                            proxy_to: route.proxy_to,
                            template: route.template,
                        },
                    )
                })
                .collect(),
        };

        result
    }
}

#[cfg(feature = "ssr")]
fn find_nginx_setup(
    src: &mut Vec<crate::nginx_http_client::NginxSetupHttpModel>,
    domain: &str,
) -> Option<NginxConfigHttpModel> {
    let index = src.iter().position(|itm| itm.domain == domain)?;

    let result = src.remove(index);

    let result = NginxConfigHttpModel {
        ca: result.client_cert_ca_cn,
        template: convert_templates(result.templates),
        routes: result
            .locations
            .into_iter()
            .map(|itm| {
                (
                    itm.location,
                    NginxRouteHttpModel {
                        proxy_to: itm.proxy_pass,
                        template: convert_templates(itm.templates),
                    },
                )
            })
            .collect(),
    };
    Some(result)
}

#[cfg(feature = "ssr")]
fn convert_templates(templates: Option<Vec<String>>) -> Option<String> {
    let mut templates = templates?;

    if templates.len() == 0 {
        return None;
    }

    let result = templates.remove(0);

    Some(result)
}
