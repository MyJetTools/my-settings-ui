use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use serde::*;
#[derive(Props, PartialEq, Eq)]
pub struct ShowSecretUsageProps {
    pub secret: String,
}

pub fn show_secret_usage_by_template<'s>(cx: Scope<'s, ShowSecretUsageProps>) -> Element {
    let secret_usage_state: &UseState<Option<Vec<TemplateUsageApiModel>>> = use_state(cx, || None);

    match secret_usage_state.get() {
        Some(values) => {
            let to_render = values.into_iter().map(|itm| {
                let items = itm.yaml.split("\n").map(|itm| {
                    if itm.contains(cx.props.secret.as_str()) {
                        rsx! {
                            div { style: "color:black;", itm }
                        }
                    } else {
                        rsx! {
                            div { style: "color:lightgray", itm }
                        }
                    }
                });

                rsx! {
                    h4 { "{itm.env}/{itm.name}" }
                    items,
                    hr {}
                }
            });

            render! {
                div { class: "modal-content",
                    div { class: "form-control modal-content-full-screen", to_render }
                }
            }
        }
        None => {
            let secret_id = cx.props.secret.to_string();

            let secret_usage_state = secret_usage_state.to_owned();

            cx.spawn(async move {
                let result = load_secret_usage(secret_id).await.unwrap();
                secret_usage_state.set(Some(result));
            });

            render! { div { class: "modal-content", "Loading..." } }
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
    let response = crate::grpc_client::SecretsGrpcClient::get_usage_of_templates(secret_id)
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
