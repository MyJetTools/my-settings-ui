use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_utils::*;

use crate::models::*;

#[component]
pub fn SelectSecret(
    env_id: Rc<String>,
    product_id: String,
    on_selected: EventHandler<String>,
) -> Element {
    let mut component_state = use_signal(|| SelectSecretState::default());

    let component_state_read_access = component_state.read();

    let secrets = match component_state_read_access.secrets.as_ref() {
        RenderState::None => {
            let env_id = env_id.to_string();
            let product_id = if product_id.len() == 0 {
                None
            } else {
                Some(product_id.to_string())
            };
            spawn(async move {
                component_state.write().secrets.set_loading();
                match crate::api::secrets::load_secrets(env_id, product_id).await {
                    Ok(data) => {
                        component_state
                            .write()
                            .secrets
                            .set_loaded(data.into_iter().map(Rc::new).collect());
                    }
                    Err(err) => {
                        component_state.write().secrets.set_error(err.to_string());
                    }
                }
            });
            return crate::icons::loading_icon();
        }
        RenderState::Loading => {
            return crate::icons::loading_icon();
        }
        RenderState::Loaded(items) => items,
        RenderState::Error(err) => {
            return crate::icons::render_error(err.as_str());
        }
    };

    let content = secrets
        .iter()
        .filter(|itm| component_state_read_access.filter_it(itm))
        .map(|itm| {
            let itm = itm.clone();
            rsx! {
                div {
                    button {
                        class: "btn btn-sm btn-primary",
                        onclick: move |_| {
                            on_selected.call(itm.secret_id.to_string());
                        },
                        "Copy Value"
                    }
                    "{itm.secret_id.as_str()}"
                }
            }
        });

    rsx! {
        input {
            class: "form-control",
            placeholder: "Type secret to fine",
            oninput: move |cx| {
                component_state.write().filter = cx.value().to_lowercase();
            },
        }

        div { style: "height:300px; overflow-y: auto;", {content} }
    }
}

#[derive(Default)]
pub struct SelectSecretState {
    secrets: DataState<Vec<Rc<SecretHttpModel>>>,
    filter: String,
}

impl SelectSecretState {
    pub fn filter_it(&self, item: &SecretHttpModel) -> bool {
        if self.filter.len() < 3 {
            return false;
        }

        item.secret_id.to_lowercase().contains(self.filter.as_str())
    }
}
