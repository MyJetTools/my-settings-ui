use std::rc::Rc;

use dioxus::prelude::*;

use dioxus_utils::{DataState, RenderState};

use crate::{dialogs::*, models::*};

#[component]
pub fn ShowSecretUsageBySecret(
    env_id: Rc<String>,
    product_id: Option<String>,
    secret_id: Rc<String>,
) -> Element {
    let mut component_state = use_signal(|| ShowSecretUsageBySecretState::new());

    let component_state_read_state = component_state.read();

    let values = match component_state_read_state.data.as_ref() {
        RenderState::None => {
            let env_id = env_id.clone();
            let product_id = product_id.clone();
            let secret_id = secret_id.to_string();
            spawn(async move {
                match crate::api::secrets::load_secret_usage_by_secret(
                    env_id.to_string(),
                    product_id.map(|itm| itm.to_string()),
                    secret_id,
                )
                .await
                {
                    Ok(result) => {
                        component_state.write().data.set_loaded(result);
                    }
                    Err(err) => {
                        component_state.write().data.set_error(err.to_string());
                    }
                }
            });
            return crate::icons::loading_icon();
        }
        RenderState::Loading => {
            return crate::icons::loading_icon();
        }
        RenderState::Loaded(data) => data,
        RenderState::Error(err) => {
            return crate::icons::render_error(err);
        }
    };

    let to_render = values.into_iter().map(|itm| {
        let index = itm.value.find(secret_id.as_str());

        match index {
            Some(index) => {
                let left = &itm.value[..index];
                let mid = &secret_id;
                let right = &itm.value[index + mid.len()..];
                rsx! {
                    tr {
                        td { "{itm.secret_id}:" }
                        td {
                            div { style: "color:gray; padding-left:5px",
                                "{left}"
                                span { style: "color:black", "{mid}" }
                                span { style: "color:gray", "{right}" }
                            }
                        }
                    }
                }
            }
            None => {
                rsx! {
                    tr {

                        td { "{itm.secret_id}:" }
                        td {
                            div { style: "color:gray; padding-left:5px", " {itm.value}" }
                        }
                    }
                }
            }
        }
    });

    rsx! {

        DialogTemplate {
            header: format!("Usage of secret {}", secret_id.as_str()),
            width: "95%",
            content: rsx! {
                div { style: "text-align:left", class: "dialog-max-content", {to_render} }
            },
        }
    }
}

pub struct ShowSecretUsageBySecretState {
    pub data: DataState<Vec<SecretUsageBySecretApiModel>>,
}

impl ShowSecretUsageBySecretState {
    pub fn new() -> Self {
        Self {
            data: DataState::new(),
        }
    }
}
