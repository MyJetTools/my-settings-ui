use dioxus::prelude::*;

use crate::{
    states::{DialogState, DialogType},
    views::dialog::*,
};

pub fn render_dialog(cx: Scope) -> Element {
    let dialog = use_shared_state::<DialogState>(cx).unwrap();

    let dialog = dialog.read();

    let dialog = match dialog.as_ref() {
        DialogState::Hidden => None,
        DialogState::Shown {
            header,
            dialog_type,
        } => {

            let mut dialog_class = "modal-dialog";
            let dialog_content = match dialog_type {
                DialogType::AddSecret => {
                    dialog_class = "modal-dialog-narrow";
                    rsx! { edit_secret { secret: "".to_string() } }
                }
                DialogType::EditSecret(secret) => {
                    dialog_class = "modal-dialog-narrow";
                    rsx! { edit_secret { secret: secret.clone() } }
                }
                DialogType::DeleteSecret(secret) => {
                    dialog_class = "modal-dialog-narrow";
                    rsx! { delete_secret_dialog { secret: secret.clone() } }
                }

                DialogType::AddTemplate => {
                    rsx! {edit_template { env: "".to_string(), name: "".to_string(), copy_from_template: false }}
                }

                DialogType::AddTemplateFromOtherTemplate{env, name} => {
                    rsx! { edit_template { env: env.clone(), name: name.clone(), copy_from_template: true } }
                }

                DialogType::EditTemplate { env, name } => {
                    rsx! {edit_template { env: env.to_string(), name: name.to_string(), copy_from_template: false }}
                }

                DialogType::DeleteTemplate { env, name } => {
                    dialog_class = "modal-dialog-narrow";
                    rsx! { delete_template_dialog { env: env.to_string(), name: name.to_string() } }
                }

                DialogType::ShowPopulatedYaml { env, name } => {
                    rsx! { show_populated_yaml { env: env.to_string(), name: name.to_string() } }
                }

                DialogType::ShowSecret(secret) => {
                    let secret = secret.clone();
                    rsx! { show_secret { secret: secret } }
                }

                DialogType::SecretUsage(secret) => {
                    let secret = secret.clone();
                    rsx! { show_secret_usage_by_template { secret: secret } }
                }

                DialogType::SecretUsageBySecret(secret) => {
                    let secret = secret.clone();
                    rsx! { show_secret_usage_by_secret { secret: secret } }
                }

                DialogType::EditDomainMask(mask)=>{
                    dialog_class = "modal-dialog-narrow";
                    rsx! { edit_domain_mask { mask: mask.to_string() } }
                }

                DialogType::AddDomainProduct=>{
                    dialog_class = "modal-dialog-narrow";
                    rsx! {edit_domain_product { add: true, name: "".to_string(), is_cloud_flare_proxy_pass: false, internal_domain_name: "".to_string() }}
                }

                DialogType::EditDomainProduct { name, cloud_flare_proxy_pass, internal_domain_name }=>{
                    dialog_class = "modal-dialog-narrow";
                    rsx! {
                        edit_domain_product {
                            add: false,
                            name: name.clone(),
                            is_cloud_flare_proxy_pass: *cloud_flare_proxy_pass,
                            internal_domain_name: internal_domain_name.clone()
                        }
                    }
                }

                DialogType::EditCfDomainRecord{domain, proxied}=>{
                    dialog_class = "modal-dialog-narrow";
                    rsx! { EditCfRecord { domain: domain.clone(), proxied: *proxied } }
                }
            };

            rsx! {
                div { id: "dialog-pad",

                    div { class: "{dialog_class}",
                        div { class: "modal-content",
                            div { class: "modal-header",
                                h5 { class: "modal-title", "{header}" }
                                button {
                                    r#type: "button",
                                    class: "btn btn-default btn-sm",
                                    onclick: move |_| {
                                        use_shared_state::<DialogState>(cx).unwrap().write().hide_dialog();
                                    },
                                    "X"
                                }
                            }
                            dialog_content
                        }
                    }
                }
            }
        }
        .into(),
    };

    render!(dialog)
}
