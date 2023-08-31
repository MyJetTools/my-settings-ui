use dioxus::prelude::*;

use crate::secrets_grpc::SecretListItem;

#[derive(Props)]
pub struct ChooseSecretProps<'s> {
    pub on_selected: EventHandler<'s, String>,
}

pub fn choose_secret<'s>(cx: Scope<'s, ChooseSecretProps<'s>>) -> Element {
    let filter = use_state(cx, || "".to_string());

    let secrets: &UseState<Option<Vec<SecretListItem>>> = use_state(cx, || None);

    if secrets.get().is_none() {
        load_secrets(&cx, secrets);
    }

    let values = match secrets.get() {
        Some(values) => {
            if filter.get().len() < 3 {
                vec![]
            } else {
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
                                    "Copy"
                                }
                                "{value.name}"
                            }
                        })
                    }
                }

                result
            }
        }
        None => vec![],
    };

    render! {
        div { style: "margin-top:5px",
            input {
                class: "form-control",
                placeholder: "Search secret",
                oninput: move |cx| {
                    filter.set(cx.value.to_lowercase());
                }
            }
        }

        values.into_iter()
    }
}

fn load_secrets<'s>(
    cx: &Scope<'s, ChooseSecretProps<'s>>,
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
