use std::collections::HashMap;

use dioxus::prelude::*;

use crate::secrets_grpc::{SecretListItem, SecretModel};

#[derive(Props, PartialEq, Eq)]
pub struct PeekSecretsProps {
    pub yaml: String,
}
pub fn peek_secrets<'s>(cx: Scope<'s, PeekSecretsProps>) -> Element {
    let available_secrets: &UseState<Option<HashMap<String, SecretListItem>>> =
        use_state(cx, || None);

    load_available_secrets(cx, available_secrets);

    let mut secrets = Vec::new();

    let secrets_state: &UseState<HashMap<String, SecretModel>> = use_state(cx, || HashMap::new());

    if cx.props.yaml.len() > 10 {
        for secret_name in settings_utils::placeholders::get_secret_names(cx.props.yaml.as_str()) {
            let secret_name_to_load = secret_name.to_string();

            if let Some(available_secrets) = available_secrets.get() {
                let (secret_value, secret_level) = if !available_secrets.contains_key(secret_name) {
                    (
                        rsx! { div { span{ class:"badge text-bg-danger", "Secret not found"} } },
                        rsx! {div{}},
                    )
                } else {
                    match secrets_state.get().get(secret_name) {
                        Some(value) => (
                            rsx! { div { style:"font-size:12px; width:300px; height:32px; overflow-y: auto;", "{value.value}" } },
                            rsx! {div{ style:"font-size:12px", "{value.level}"}},
                        ),
                        None => (
                            rsx! { div{class:"btn-group", button { class:"btn btn-primary btn-sm", onclick: move|_|{
                                load_secret(cx, &secret_name_to_load, secrets_state);
                            }, "Load" } }},
                            rsx! {div{}},
                        ),
                    }
                };

                secrets.push(rsx! {
                    tr { style: "border-top: 1px solid lightgray",
                        td { style: "font-size:12px; border-right: 1px solid lightgray",
                            "{secret_name}:"
                        }
                        td { width: "100%", secret_value }
                        td { width: "30px", secret_level }
                    }
                });
            }
        }
    }

    render! {
        div { style: "height:70vh; overflow-y: auto;",
            table { class: "table table-striped",
                tr {
                    th { "secret" }
                    th { "value" }
                    th { "level" }
                }
                secrets.into_iter()
            }
        }
    }
}

pub fn load_available_secrets<'s>(
    cx: &'s Scoped<'s, PeekSecretsProps>,
    available_secrets: &UseState<Option<HashMap<String, SecretListItem>>>,
) {
    if available_secrets.get().is_some() {
        return;
    }
    let available_secrets = available_secrets.to_owned();
    cx.spawn(async move {
        let as_vec = crate::grpc_client::SecretsGrpcClient::get_all_secrets()
            .await
            .unwrap();

        let mut values = HashMap::new();

        for itm in as_vec {
            values.insert(itm.name.clone(), itm);
        }

        available_secrets.set(Some(values));
    });
}

pub fn load_secret<'s>(
    cx: &'s Scoped<'s, PeekSecretsProps>,
    secret_name: &str,
    secrets: &UseState<HashMap<String, SecretModel>>,
) {
    let secret_name = secret_name.to_string();
    let secrets = secrets.to_owned();

    cx.spawn(async move {
        let secret_model = crate::grpc_client::SecretsGrpcClient::get_secret(secret_name.clone())
            .await
            .unwrap();

        if secret_model.name.len() > 0 {
            secrets.modify(|itm| {
                let mut secrets = itm.clone();
                secrets.insert(secret_name, secret_model);
                secrets
            });
        }
    });
}
