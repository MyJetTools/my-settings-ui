use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_utils::DataState;

use crate::{icons::*, models::*};

#[component]
pub fn SelectSecret(env_id: Rc<String>, on_selected: EventHandler<String>) -> Element {
    let mut component_state = use_signal(|| SelectSecretState::new());

    let component_state_read_access = component_state.read();

    let secrets = match component_state_read_access.secrets.as_ref() {
        DataState::None => {
            let env_id = env_id.clone();
            spawn(async move {
                component_state.write().secrets = DataState::Loading;
                match crate::api::secrets::load_secrets(env_id.to_string()).await {
                    Ok(data) => {
                        component_state.write().secrets =
                            DataState::Loaded(data.into_iter().map(Rc::new).collect());
                    }
                    Err(err) => {
                        component_state.write().secrets = DataState::Error(err.to_string());
                    }
                }
            });
            return rsx! {
                LoadingIcon {}
            };
        }
        DataState::Loading => {
            return rsx! {
                LoadingIcon {}
            }
        }
        DataState::Loaded(items) => items,
        DataState::Error(err) => {
            return rsx! {
                div { {err.as_str()} }
            };
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
                            on_selected.call(itm.name.to_string());
                        },
                        "Copy Value"
                    }
                    "{itm.name.as_str()}"
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

pub struct SelectSecretState {
    secrets: DataState<Vec<Rc<SecretHttpModel>>>,
    filter: String,
}

impl SelectSecretState {
    pub fn new() -> Self {
        Self {
            secrets: DataState::None,
            filter: String::new(),
        }
    }

    pub fn filter_it(&self, item: &SecretHttpModel) -> bool {
        if self.filter.len() < 3 {
            return false;
        }

        item.name.to_lowercase().contains(self.filter.as_str())
    }
}
