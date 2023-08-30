use dioxus::prelude::*;

use crate::secrets_grpc::SecretUsageModel;

#[derive(Props, PartialEq, Eq)]
pub struct ShowSecretUsageProps {
    pub secret: String,
}

pub fn show_secret_usage_by_secret<'s>(cx: Scope<'s, ShowSecretUsageProps>) -> Element {
    let secret_usage: &UseState<Option<Vec<SecretUsageModel>>> = use_state(cx, || None);

    match secret_usage.get() {
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
            load_secret_usage(&cx, cx.props.secret.clone(), secret_usage);

            render! { div { class: "modal-content", "Loading..." } }
        }
    }
}

fn load_secret_usage<'s>(
    cx: &Scope<'s, ShowSecretUsageProps>,
    secret_id: String,
    state: &UseState<Option<Vec<SecretUsageModel>>>,
) {
    let state = state.to_owned();

    cx.spawn(async move {
        let response = crate::grpc_client::SecretsGrpcClient::get_usage_of_secrets(secret_id)
            .await
            .unwrap();

        state.set(Some(response))
    });
}
