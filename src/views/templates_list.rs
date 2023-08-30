use std::rc::Rc;

use dioxus::prelude::*;
use rust_extensions::date_time::DateTimeAsMicroseconds;

use super::icons::*;
use crate::states::{DialogState, DialogType, LastEdited, MainState};

pub fn templates_list(cx: Scope) -> Element {
    let main_state = use_shared_state::<MainState>(cx).unwrap();

    let last_edited = use_shared_state::<LastEdited>(cx)
        .unwrap()
        .read()
        .get_template();

    match main_state.read().unwrap_as_templates() {
        Some(templates) => {
            let templates = templates.iter().map(|itm| {
                let last_request = if itm.last_requests == 0 {
                    "".to_string()
                } else {
                    let last_request = DateTimeAsMicroseconds::new(itm.last_requests * 1000);
                    last_request.to_rfc3339()
                };

                let env = Rc::new(itm.env.to_string());
                let name = Rc::new(itm.name.to_string());

                let show_populated_yaml_env = env.clone();
                let show_populated_yaml_name = name.clone();

                let delete_template_env = env.clone();
                let delete_template_name = name.clone();

                let last_edited = if last_edited.0.as_str() == env.as_str() && last_edited.1.as_str() == name.as_str(){
                    Some(rsx!(
                        span { id: "last-edited-badge", class: "badge badge-success ", "Last edited" }
                        script { r#"scroll_to('last-edited-badge')"# }
                    ))
                }else{
                    None
                };

                rsx! {
                    tr { style: "border-top: 1px solid lightgray",
                        td { "{itm.env}" }
                        td { "{itm.name}", last_edited }
                        td { "{itm.created}" }
                        td { "{itm.updated}" }
                        td { "{last_request}" }
                        td {
                            div { class: "btn-group",
                                button {
                                    class: "btn btn-sm btn-success",
                                    onclick: move |_| {
                                        let dialog_state = use_shared_state::<DialogState>(cx).unwrap();
                                        dialog_state
                                            .write()
                                            .show_dialog(
                                                format!(
                                                    "{}/{}", show_populated_yaml_env.as_str(), show_populated_yaml_name
                                                    .as_str()
                                                ),
                                                DialogType::ShowPopulatedYaml {
                                                    env: show_populated_yaml_env.to_string(),
                                                    name: show_populated_yaml_name.to_string(),
                                                },
                                            );
                                    },
                                    view_template_icon {}
                                }
                                button {
                                    class: "btn btn-sm btn-primary",
                                    onclick: move |_| {
                                        let dialog_state = use_shared_state::<DialogState>(cx).unwrap();
                                        dialog_state
                                            .write()
                                            .show_dialog(
                                                "Edit template".to_string(),
                                                DialogType::EditTemplate {
                                                    env: env.to_string(),
                                                    name: name.to_string(),
                                                },
                                            );
                                    },
                                    edit_icon {}
                                }
                                button {
                                    class: "btn btn-sm btn-danger",
                                    onclick: move |_| {
                                        let dialog_state = use_shared_state::<DialogState>(cx).unwrap();
                                        dialog_state
                                            .write()
                                            .show_dialog(
                                                format!(
                                                    "Delete template {}/{}", delete_template_env.as_str(),
                                                    delete_template_name.as_str()
                                                ),
                                                DialogType::DeleteTemplate {
                                                    env: delete_template_env.to_string(),
                                                    name: delete_template_name.to_string(),
                                                },
                                            );
                                    },
                                    delete_icon {}
                                }
                            }
                        }
                    }
                }
            });
            render! {
                table { class: "table table-striped", style: "text-align: left;",
                    tr {
                        th { "Env" }
                        th { "Name" }
                        th { "Created" }
                        th { "Updated" }
                        th { "Last request" }
                        th {
                            div {
                                button {
                                    class: "btn btn-sm btn-primary",
                                    onclick: |_| {
                                        let dialog_state = use_shared_state::<DialogState>(cx).unwrap();
                                        dialog_state
                                            .write()
                                            .show_dialog("Add template".to_string(), DialogType::AddTemplate);
                                    },
                                    add_icon {}
                                }
                            }
                        }
                    }

                    templates.into_iter()
                }
            }
        }
        None => {
            load_templates(&cx, &main_state);
            render! { h1 { "loading" } }
        }
    }
}

fn load_templates(cx: &Scope, main_state: &UseSharedState<MainState>) {
    let main_state = main_state.to_owned();

    cx.spawn(async move {
        let response = crate::grpc_client::TemplatesGrpcClient::get_all_templates()
            .await
            .unwrap();

        main_state.write().set_templates(Some(response));
    });
}
