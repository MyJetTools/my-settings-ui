use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_utils::{DataState, RenderState};

use crate::dialogs::*;

#[component]
pub fn ShowSecret(
    env_id: Rc<String>,
    product_id: Option<Rc<String>>,
    secret_id: Rc<String>,
) -> Element {
    let mut component_state = use_signal(|| ShowSecretState::new());

    let component_state_read_access = component_state.read();

    let content = match component_state_read_access.value.as_ref() {
        RenderState::None => {
            let env_id = env_id.clone();
            let product_id = product_id.clone();
            let secret_id = secret_id.clone();
            spawn(async move {
                component_state.write().value.set_loading();
                match crate::api::secrets::load_secret_value(
                    env_id.to_string(),
                    product_id.map(|itm| itm.to_string()),
                    secret_id.to_string(),
                )
                .await
                {
                    Ok(value) => {
                        component_state.write().value.set_loaded(value.value);
                    }
                    Err(err) => {
                        component_state.write().value.set_error(err.to_string());
                    }
                }
            });
            return crate::icons::loading_icon();
        }
        RenderState::Loading => {
            return crate::icons::loading_icon();
        }
        RenderState::Loaded(value) => rsx! {
            div { class: "form-floating mb-3",
                input {
                    class: "form-control",
                    readonly: true,
                    value: value.as_str(),
                }
                label { "Secret value" }
            }
        },
        RenderState::Error(err) => {
            return crate::icons::render_error(err);
        }
    };

    rsx! {
        DialogTemplate { header: "Secret [{secret_id.as_str()}] value", content }
    }
}

pub struct ShowSecretState {
    pub value: DataState<String>,
}

impl ShowSecretState {
    pub fn new() -> Self {
        Self {
            value: DataState::new(),
        }
    }
}
