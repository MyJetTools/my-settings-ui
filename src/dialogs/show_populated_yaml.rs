use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_utils::DataState;
use serde::*;

use crate::{dialogs::*, views::icons::*};

#[component]
pub fn ShowPopulatedYaml(env_id: Rc<String>, env: Rc<String>, name: Rc<String>) -> Element {
    let mut component_state = use_signal(|| ShowPopulatedYamlState::new());

    let component_state_read_access = component_state.read();

    let content = match component_state_read_access.yaml.as_ref() {
        DataState::None => {
            let env_id = env_id.clone();
            let env = env.to_string();
            let name = name.to_string();
            spawn(async move {
                match load_yaml(env_id.to_string(), env, name).await {
                    Ok(result) => {
                        component_state.write().yaml = DataState::Loaded(result.yaml);
                    }
                    Err(err) => {
                        component_state.write().yaml = DataState::Error(err.to_string());
                    }
                }
            });
            rsx! {}
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
async fn load_yaml(
    env_id: String,
    env: String,
    name: String,
) -> Result<PopulatedYamlModelApiModel, ServerFnError> {
    use crate::server::templates_grpc::*;
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    let response = ctx
        .templates_grpc
        .compile_yaml(CompileYamlRequest { env, name })
        .await
        .unwrap();

    Ok(PopulatedYamlModelApiModel {
        yaml: response.yaml,
    })
}
