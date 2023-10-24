use crate::{
    states::{DialogState, MainState},
    views::icons::*,
};
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

#[derive(Props, PartialEq, Eq)]
pub struct DeleteTemplateProps {
    pub env: String,
    pub name: String,
}
pub fn delete_template_dialog<'s>(cx: Scope<'s, DeleteTemplateProps>) -> Element {
    let content = format!(
        "You are about to delete a template {}/{}",
        cx.props.env, cx.props.name
    );
    render! {
        div { class: "modal-content",
            h4 { content }
        }
        div { class: "modal-footer",
            div { class: "btn-group",
                button {
                    class: "btn btn-primary",
                    onclick: move |_| {
                        let env = cx.props.env.to_string();
                        let name = cx.props.env.to_string();
                        let main_state = use_shared_state::<MainState>(cx).unwrap().to_owned();
                        let dialog_state: UseSharedState<DialogState> = use_shared_state::<DialogState>(cx)
                            .unwrap()
                            .to_owned();
                        cx.spawn(async move {
                            delete_template(env, name).await.unwrap();
                            dialog_state.write().hide_dialog();
                            main_state.write().set_templates(None);
                        });
                    },
                    ok_button_icon {}
                    "Save"
                }
                button {
                    class: "btn btn-outline-dark",
                    onclick: move |_| {
                        use_shared_state::<DialogState>(cx).unwrap().write().hide_dialog();
                    },
                    cancel_button_icon {}
                    "Cancel"
                }
            }
        }
    }
}

#[server]
async fn delete_template(env: String, name: String) -> Result<(), ServerFnError> {
    crate::grpc_client::TemplatesGrpcClient::delete_template(env, name)
        .await
        .unwrap();

    Ok(())
    /*
    let env = cx.props.env.clone();
    let name = cx.props.name.clone();

    let main_state = use_shared_state::<MainState>(cx).unwrap().to_owned();
    let dialog_state: UseSharedState<DialogState> =
        use_shared_state::<DialogState>(cx).unwrap().to_owned();

    cx.spawn(async move {


        dialog_state.write().hide_dialog();
        main_state.write().set_templates(None);
    })
     */
}
