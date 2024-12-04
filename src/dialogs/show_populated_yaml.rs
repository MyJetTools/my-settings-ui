use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_utils::DataState;
use serde::*;

use crate::{dialogs::*, views::icons::*};

#[component]
pub fn ShowPopulatedYaml(env: Rc<String>, name: Rc<String>) -> Element {
    let mut component_state = use_signal(|| ShowPopulatedYamlState::new());

    let component_state_read_access = component_state.read();

    let content = match component_state_read_access.yaml.as_ref() {
        DataState::None => {
            let env = env.to_string();
            let name = name.to_string();
            spawn(async move {
                match load_yaml(env, name).await {
                    Ok(result) => {
                        component_state.write().yaml = DataState::Loaded(result.yaml);
                    }
                    Err(err) => {
                        component_state.write().yaml = DataState::Error(err.to_string());
                    }
                }
            });
            None
        }
        DataState::Loading => {
            rsx! {
                LoadingIcon {}
            }
        }
        DataState::Loaded(yaml) => {
            rsx! {
                textarea {
                    class: "form-control modal-content-full-screen",
                    readonly: true,
                    {yaml.as_str()}
                }
            }
        }
        DataState::Error(err) => rsx! {
            div { {err.as_str()} }
        },
    };

    rsx! {
        DialogTemplate { header: "Populated yaml", allocate_max_space: true, content }
    }
}

pub struct ShowPopulatedYamlState {
    pub yaml: DataState<String>,
}

impl ShowPopulatedYamlState {
    pub fn new() -> Self {
        Self {
            yaml: DataState::None,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PopulatedYamlModelApiModel {
    pub yaml: String,
}

#[server]
async fn load_yaml<'s>(
    env: String,
    name: String,
) -> Result<PopulatedYamlModelApiModel, ServerFnError> {
    let yaml = crate::server::grpc_client::TemplatesGrpcClient::get_populated_template(env, name)
        .await
        .unwrap();

    Ok(PopulatedYamlModelApiModel { yaml })
}
