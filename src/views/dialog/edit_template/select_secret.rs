use dioxus::prelude::*;

use crate::views::{load_secrets, SecretListItemApiModel};

#[derive(Props)]
pub struct SelectSecretProps<'s> {
    pub on_selected: EventHandler<'s, String>,
}

pub fn select_secret<'s>(cx: Scope<'s, SelectSecretProps<'s>>) -> Element {
    let secrets_state = use_state::<Option<Vec<SecretListItemApiModel>>>(cx, || None);
    let filter = use_state(cx, || "".to_string());

    if secrets_state.get().is_none() {
        let secrets_state = secrets_state.to_owned();
        cx.spawn(async move {
            let result = load_secrets().await.unwrap();
            secrets_state.set(Some(result));
        })
    }

    let content = match secrets_state.get() {
        Some(values) => {
            if filter.len() > 3 {
                let mut result = Vec::new();

                for value in values {
                    if value.name.to_lowercase().contains(filter.get().as_str()) {
                        result.push(rsx! {
                            div {
                                button {
                                    class: "btn btn-sm btn-primary",
                                    onclick: move |_| {
                                        cx.props.on_selected.call(value.name.to_string());
                                    },
                                    "Copy Value"
                                }
                                "{value.name}"
                            }
                        })
                    }
                }

                result
            } else {
                vec![]
            }
        }
        None => vec![],
    };

    render! {
        input {
            class: "form-control",
            placeholder: "Type secret to fine",
            oninput: move |cx| {
                filter.set(cx.value.to_lowercase());
            }
        }

        div { style: "height:300px; overflow-y: auto;", content.into_iter() }
    }
}
