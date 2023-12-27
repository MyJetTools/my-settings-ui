use std::rc::Rc;

use dioxus::{html::GlobalAttributes, prelude::*};
use dioxus_fullstack::prelude::*;
use serde::*;

use crate::{
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

    let products = widget_state_value.products.iter().map(|itm| {
        let name = itm.name.clone();
        let cloud_flare_proxy_pass = itm.is_cloud_flare_proxy_pass;
        let internal_domain_name = itm.internal_domain_name.clone();

        let product_domain_name = Rc::new(domain_mask.replace("*", &itm.name));

        let proxy_pass = itm.is_cloud_flare_proxy_pass;

        rsx! {
            tr { style: "border-bottom: 1px solid lightgray; text-align: left;",

                td { "{product_domain_name.as_str()}" }
                td { ProxyPassIcon { proxy_pass: proxy_pass, height: 32 } }
                td { "{itm.internal_domain_name}" }
                td {
                    RenderCloudFlareStatus {
                        domain: product_domain_name.clone(),
                        ip: "127.0.0.1".to_string(),
                        proxied: proxy_pass
                    }
                }
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
                                            internal_domain_name: internal_domain_name.clone(),
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
                th { "Internal domain name" }
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
        table {
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
            }
        }

        h2 { "Product domains:" }

        product_domains
    }
}

#[component]
fn RenderCloudFlareStatus(cx: Scope, domain: Rc<String>, ip: String, proxied: bool) -> Element {
    let domains_state = use_shared_state::<CloudFlareRecordsState>(cx).unwrap();

    let domains_state = domains_state.read();

    let domains_state = domains_state.get_value();

    if domains_state.is_none() {
        return render! { div { class: "alert alert-warning", "Loading Cloudfalre info..." } };
    }

    let domains_state = domains_state.as_ref().unwrap();

    let result = match domains_state.get(domain.as_ref()) {
        Some(value) => {
            if value.tp != "A" {
                Some("Not A-record")
            } else if &value.content != ip {
                Some("Invalid IP")
            } else {
                None
            }
        }
        None => Some("No Cloudflare record found"),
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
                                },
                            );
                    },
                    "{err}"
                }
            }
        },
        None => render! { div { class: "badge bg-success", "OK" } },
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainsApiModel {
    pub domain_mask: Option<String>,
    pub products: Vec<DomainProduct>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainProduct {
    pub name: String,
    pub is_cloud_flare_proxy_pass: bool,
    pub internal_domain_name: String,
}

#[server]
pub async fn load_domains() -> Result<DomainsApiModel, ServerFnError> {
    let response = crate::grpc_client::DomainsGrpcClient::get().await.unwrap();

    let result = DomainsApiModel {
        domain_mask: response.domain_mask,
        products: response
            .products
            .into_iter()
            .map(|itm| DomainProduct {
                name: itm.product_name,
                is_cloud_flare_proxy_pass: itm.is_cloud_flare_proxy,
                internal_domain_name: itm.internal_domain_name,
            })
            .collect(),
    };

    Ok(result)
}

#[server]
async fn get_cf_records(domain: String) -> Result<Vec<CfRecordRestApiModel>, ServerFnError> {
    let cloud_flare_bridge_url = crate::APP_CTX.settings.get_cloud_flare_url();

    let mut response = flurl::FlUrl::new(cloud_flare_bridge_url)
        .append_path_segment("api")
        .append_path_segment("DnsZone")
        .append_query_param("domain", Some(domain))
        .get()
        .await
        .unwrap();

    let response: Vec<CfRecordRestApiModel> = response.get_json().await.unwrap();

    Ok(response)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CfRecordRestApiModel {
    pub id: String,
    pub name: String,
    pub tp: String,
    pub content: String,
    pub proxied: bool,
}
