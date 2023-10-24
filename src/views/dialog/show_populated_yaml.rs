use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use serde::*;

#[derive(Props, PartialEq, Eq)]
pub struct ShowPopulatedYamlProps {
    pub env: String,
    pub name: String,
}

pub fn show_populated_yaml<'s>(cx: Scope<'s, ShowPopulatedYamlProps>) -> Element {
    let yaml_state: &UseState<Option<String>> = use_state(cx, || None);

    match yaml_state.get() {
        Some(yaml) => {
            render! {
                div { class: "modal-content",
                    textarea { class: "form-control modal-content-full-screen", readonly: true, yaml.as_str() }
                }
            }
        }
        None => {
            let name = cx.props.name.to_string();
            let env = cx.props.env.to_string();
            let yaml_state = yaml_state.to_owned();
            cx.spawn(async move {
                let result = load_yaml(env, name).await.unwrap();
                yaml_state.set(Some(result.yaml));
            });
            return render! {
                div { class: "modal-content", div { class: "form-control modal-content-full-screen", "Loading..." } }
            };
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
    let yaml = crate::grpc_client::TemplatesGrpcClient::get_populated_template(env, name)
        .await
        .unwrap();

    Ok(PopulatedYamlModelApiModel { yaml })
}
