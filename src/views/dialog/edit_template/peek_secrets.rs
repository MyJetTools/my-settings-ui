use std::collections::HashMap;

use dioxus::prelude::*;

use crate::secrets_grpc::SecretModel;

#[derive(Props, PartialEq, Eq)]
pub struct PeekSecretsProps {
    pub yaml: String,
}
pub fn peek_secrets<'s>(cx: Scope<'s, PeekSecretsProps>) -> Element {
    let mut secrets = Vec::new();

    let secrets_state: &UseState<HashMap<String, Option<SecretModel>>> =
        use_state(cx, || HashMap::new());

    if cx.props.yaml.len() > 10 {
        for secret_name in settings_utils::placeholders::get_secret_names(cx.props.yaml.as_str()) {
            let secret_name_to_load = secret_name.to_string();
            let (secret_value, secret_level) = match secrets_state.get().get(secret_name) {
                Some(value) => match value {
                    Some(value) => (
                        rsx! { div { style:"font-size:12px; width:300px; height:32px; overflow-y: auto;", "{value.value}" } },
                        rsx! {div{ style:"font-size:12px", "{value.level}"}},
                    ),
                    None => (
                        rsx! { div { style: "color:red", "Value not found" } },
                        rsx! {div{}},
                    ),
                },
                None => (
                    rsx! { div{class:"btn-group", button { class:"btn btn-primary btn-sm", onclick: move|_|{
                        load_secret(cx, &secret_name_to_load, secrets_state);
                    }, "Load" } }},
                    rsx! {div{}},
                ),
            };

            secrets.push(rsx! {
                tr { style: "border-top: 1px solid lightgray",
                    td { style: "font-size:12px; border-right: 1px solid lightgray", "{secret_name}:" }
                    td { width: "100%", secret_value }
                    td { width: "30px", secret_level }
                }
            });
        }
    }

    render! {
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

pub fn load_secret<'s>(
    cx: &'s Scoped<'s, PeekSecretsProps>,
    secret_name: &str,
    secrets: &UseState<HashMap<String, Option<SecretModel>>>,
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
                secrets.insert(secret_name, Some(secret_model));
                secrets
            });
        } else {
            secrets.modify(|itm| {
                let mut secrets = itm.clone();
                secrets.insert(secret_name, None);
                secrets
            });
        }
    });
}
