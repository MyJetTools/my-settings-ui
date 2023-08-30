use dioxus::prelude::*;

use crate::secrets_grpc::TemplateUsageModel;

#[derive(Props, PartialEq, Eq)]
pub struct ShowSecretUsageProps {
    pub secret: String,
}

pub fn show_secret_usage_by_template<'s>(cx: Scope<'s, ShowSecretUsageProps>) -> Element {
    let secret_usage: &UseState<Option<Vec<TemplateUsageModel>>> = use_state(cx, || None);

    match secret_usage.get() {
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
            load_secret_usage(&cx, cx.props.secret.clone(), secret_usage);

            render! { div { class: "modal-content", "Loading..." } }
        }
    }
}

fn load_secret_usage<'s>(
    cx: &Scope<'s, ShowSecretUsageProps>,
    secret_id: String,
    state: &UseState<Option<Vec<TemplateUsageModel>>>,
) {
    let state = state.to_owned();

    cx.spawn(async move {
        let response = crate::grpc_client::SecretsGrpcClient::get_usage_of_templates(secret_id)
            .await
            .unwrap();

        state.set(Some(response))
    });
}
