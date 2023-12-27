use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

use crate::{
    states::*,
    views::{dialog::edit_domain_product::state::EditDomainProductState, icons::*},
};

#[derive(Props, PartialEq, Eq)]
pub struct EditDomainProductProps {
    pub add: bool,
    pub name: String,
    pub is_cloud_flare_proxy_pass: bool,
    pub internal_domain_name: String,
}

pub fn edit_domain_product<'s>(cx: Scope<'s, EditDomainProductProps>) -> Element {
    let widget_state = use_ref(cx, || EditDomainProductState::new(cx.props));

    let save_button_is_disabled = !widget_state.read().can_be_saved();

    let cloud_flare_value = if widget_state.read().is_cloud_flare_proxy {
        "true"
    } else {
        "false"
    };

    render! {
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

            div { class: "form-floating mb-3",
                input {
                    class: "form-control",
                    value: "{widget_state.read().get_internal_domain_name()}",
                    oninput: move |cx| {
                        let mut widget_state = widget_state.write();
                        widget_state.set_internal_domain_name(cx.value.as_str());
                    }
                }
                label { "Internal domain name" }
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
                        let (product_name, is_cloud_flare_proxy, internal_domain_name) = {
                            let widget_state = widget_state.read();
                            (
                                widget_state.get_product_name().to_string(),
                                widget_state.is_cloud_flare_proxy,
                                widget_state.get_internal_domain_name().to_string(),
                            )
                        };
                        cx.spawn(async move {
                            save_domain_product(product_name, is_cloud_flare_proxy, internal_domain_name)
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

#[server]
async fn save_domain_product<'s>(
    product_name: String,
    is_cloud_flare_proxy: bool,
    internal_domain_name: String,
) -> Result<(), ServerFnError> {
    crate::grpc_client::DomainsGrpcClient::save(
        product_name,
        is_cloud_flare_proxy,
        internal_domain_name,
    )
    .await
    .unwrap();

    Ok(())
}
