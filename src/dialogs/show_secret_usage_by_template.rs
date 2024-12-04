use std::rc::Rc;

use dioxus::prelude::*;

use dioxus_utils::DataState;
use serde::*;

use crate::views::icons::*;

use super::*;

#[component]
pub fn ShowSecretUsageByTemplate(secret: Rc<String>) -> Element {
    let secret_usage_state = use_signal(|| ShowSecretUsageByTemplateState::new());

    let secret_usage_state_read_access = secret_usage_state.read();

    let data = match secret_usage_state_read_access.data.as_ref() {
        DataState::None => {
            return rsx! {
                div {}
            }
        }
        DataState::Loading => {
            return rsx! {
                LoadingIcon {}
            }
        }
        DataState::Loaded(data) => data,
        DataState::Error(err) => {
            return rsx! {
                div { {err.as_str()} }
            }
        }
    };

    let content = data.into_iter().map(|itm| {
        let items = itm.yaml.split("\n").map(|itm| {
            if itm.contains(secret.as_str()) {
                rsx! {
                    div { style: "color:black;", {itm} }
                }
            } else {
                rsx! {
                    div { style: "color:lightgray", {itm} }
                }
            }
        });

        rsx! {
            h4 { "{itm.env}/{itm.name}" }
            {items},
            hr {}
        }
    });

    rsx! {
        DialogTemplate {
            header: "Usage of secret {secret.as_str()}",
            content: rsx! {
                {content}
            }
        }
    }
}

pub struct ShowSecretUsageByTemplateState {
    data: DataState<Vec<TemplateUsageApiModel>>,
}

impl ShowSecretUsageByTemplateState {
    pub fn new() -> Self {
        Self {
            data: DataState::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateUsageApiModel {
    pub env: String,
    pub name: String,
    pub yaml: String,
}

#[server]
async fn load_secret_usage(secret_id: String) -> Result<Vec<TemplateUsageApiModel>, ServerFnError> {
    let response = crate::server::grpc_client::SecretsGrpcClient::get_usage_of_templates(secret_id)
        .await
        .unwrap();

    let result: Vec<TemplateUsageApiModel> = response
        .into_iter()
        .map(|itm| TemplateUsageApiModel {
            env: itm.env,
            name: itm.name,
            yaml: itm.yaml,
        })
        .collect();

    Ok(result)
}
