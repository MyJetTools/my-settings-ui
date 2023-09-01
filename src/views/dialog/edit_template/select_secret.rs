use dioxus::prelude::*;

use crate::secrets_grpc::SecretListItem;

#[derive(Props)]
pub struct SelectSecretProps<'s> {
    pub on_selected: EventHandler<'s, String>,
}

pub fn select_secret<'s>(cx: Scope<'s, SelectSecretProps<'s>>) -> Element {
    let secrets: &UseState<Option<Vec<SecretListItem>>> = use_state(cx, || None);
    let filter = use_state(cx, || "".to_string());

    if secrets.get().is_none() {
        load_secrets(&cx, secrets);
    }

    let content = match secrets.get() {
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

fn load_secrets<'s>(
    cx: &Scope<'s, SelectSecretProps<'s>>,
    secrets: &UseState<Option<Vec<SecretListItem>>>,
) {
    let secrets = secrets.to_owned();
    cx.spawn(async move {
        let secrets_values = crate::grpc_client::SecretsGrpcClient::get_all_secrets()
            .await
            .unwrap();

        secrets.set(Some(secrets_values));
    });
}
