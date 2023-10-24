use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use serde::*;

#[derive(Props, PartialEq, Eq)]
pub struct ShowSecretUsageProps {
    pub secret: String,
}

pub fn show_secret_usage_by_secret<'s>(cx: Scope<'s, ShowSecretUsageProps>) -> Element {
    let secret_usage_state: &UseState<Option<Vec<SecretUsageBySecretApiModel>>> =
        use_state(cx, || None);

    match secret_usage_state.get() {
        Some(values) => {
            let to_render = values.into_iter().map(|itm| {
                let index = itm.value.find(cx.props.secret.as_str());

                match index {
                    Some(index) => {
                        let left = &itm.value[..index];
                        let mid = &cx.props.secret;
                        let right = &itm.value[index + mid.len()..];
                        rsx! {
                            div { style: "color:gray",
                                "{itm.name}: {left}"
                                span { style: "color:black", "{mid}" }
                                span { style: "color:gray", "{right}" }
                            }
                        }
                    }
                    None => {
                        rsx! { div { style: "color:gray", "{itm.name}: {itm.value}" } }
                    }
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
                let result = load_secret_usage_by_secret(secret_id).await.unwrap();
                secret_usage_state.set(Some(result))
            });

            render! { div { class: "modal-content", "Loading..." } }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SecretUsageBySecretApiModel {
    pub name: String,
    pub value: String,
}

#[server]
async fn load_secret_usage_by_secret<'s>(
    secret_id: String,
) -> Result<Vec<SecretUsageBySecretApiModel>, ServerFnError> {
    let response = crate::grpc_client::SecretsGrpcClient::get_usage_of_secrets(secret_id)
        .await
        .unwrap();

    let result: Vec<_> = response
        .into_iter()
        .map(|itm| SecretUsageBySecretApiModel {
            name: itm.name,
            value: itm.value,
        })
        .collect();

    Ok(result)
}
