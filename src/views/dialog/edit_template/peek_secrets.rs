use std::collections::HashMap;

use dioxus::prelude::*;

use dioxus_fullstack::prelude::*;
use serde::*;

use crate::views::{load_secrets, SecretListItemApiModel};

#[derive(Props, PartialEq, Eq)]
pub struct PeekSecretsProps {
    pub yaml: String,
}
pub fn peek_secrets<'s>(cx: Scope<'s, PeekSecretsProps>) -> Element {
    let available_secrets: &UseState<Option<HashMap<String, SecretListItemApiModel>>> =
        use_state(cx, || None);

    if available_secrets.get().is_none() {
        let available_secrets = available_secrets.to_owned();
        cx.spawn(async move {
            let as_vec = load_secrets().await.unwrap();

            let mut values = HashMap::new();

            for itm in as_vec {
                values.insert(itm.name.clone(), itm);
            }

            available_secrets.set(Some(values));
        });
    }

    let mut secrets_to_render = Vec::new();

    let secrets_values_state: &UseState<HashMap<String, SecretApiModel>> =
        use_state(cx, || HashMap::new());

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
                    match secrets_values_state.get().get(secret_name) {
                        Some(value) => (
                            rsx! { div { style:"font-size:12px; width:300px; height:32px; overflow-y: auto;", "{value.value}" } },
                            rsx! {div{ style:"font-size:12px", "{value.level}"}},
                        ),
                        None => (
                            rsx! { div{class:"btn-group", button { class:"btn btn-primary btn-sm", onclick: move|_|{

                                let secret_name = secret_name_to_load.to_string();
                                let secrets_values_state = secrets_values_state.to_owned();
                                cx.spawn(async move{
                                    let secret_model = load_secret(secret_name).await.unwrap();
                                    if secret_model.name.as_str().len() > 0 {
                                        secrets_values_state.modify(|itm| {
                                            let mut new_secrets = itm.clone();
                                            new_secrets.insert(secret_model.name.clone(), secret_model);
                                            new_secrets
                                        });
                                    }
                                });

                            }, "Load" } }},
                            rsx! {div{}},
                        ),
                    }
                };

                secrets_to_render.push(rsx! {
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
                secrets_to_render.into_iter()
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SecretApiModel {
    pub name: String,
    pub value: String,
    pub level: i32,
}

#[server]
async fn load_secret(secret_name: String) -> Result<SecretApiModel, ServerFnError> {
    let secret_model = crate::grpc_client::SecretsGrpcClient::get_secret(secret_name.clone())
        .await
        .unwrap();

    let result = SecretApiModel {
        name: secret_name,
        value: secret_model.value,
        level: secret_model.level,
    };

    Ok(result)

    /*
    if secret_model.name.len() > 0 {
        secrets.modify(|itm| {
            let mut secrets = itm.clone();
            secrets.insert(secret_name, secret_model);
            secrets
        });
    }

    let secret_name = secret_name.to_string();
    let secrets = secrets.to_owned();

    cx.spawn(async move {});
     */
}
