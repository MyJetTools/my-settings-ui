use std::rc::Rc;

use super::icons::*;
use crate::states::{DialogState, DialogType, MainState};
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use serde::*;

pub fn templates_list(cx: Scope) -> Element {
    let main_state = use_shared_state::<MainState>(cx).unwrap();

    let filter = use_state(cx, || "".to_string());

    let value_to_filter = filter.get().to_lowercase();

    match main_state.read().unwrap_as_templates() {
        Some(templates) => {
            let last_edited = get_last_edited(templates);
            let templates = templates.iter().filter(|itm|{
                if value_to_filter.len() == 0 {
                    return true;
                }

                itm.name.to_lowercase().contains(&value_to_filter)

            }).map(|itm| {
                let last_request = if itm.last_requests == 0 {
                    "".to_string()
                } else {
                    crate::utils::unix_microseconds_to_string(itm.last_requests * 1000)
                };

                let env = Rc::new(itm.env.to_string());
                let name = Rc::new(itm.name.to_string());

                let show_populated_yaml_env = env.clone();
                let show_populated_yaml_name = name.clone();

                let delete_template_env = env.clone();
                let delete_template_name = name.clone();

                let copy_env = env.clone();
                let copy_name = name.clone();

                let last_edited = if last_edited.0.as_str() == env.as_str() && last_edited.1.as_str() == name.as_str(){
                    Some(rsx!(
                        span { id: "last-edited-badge", class: "badge badge-success ", "Last edited" }
                        script { r#"scroll_to('last-edited-badge')"# }
                    ))
                }else{
                    None
                };

                let alert = if itm.has_missing_placeholders{
                    Some(rsx!{
                        div { warning_icon {} }
                    })
                }else{
                    None
                };

      
                rsx! {
                    tr { style: "border-top: 1px solid lightgray",
                        td { alert }
                        td { "{itm.env}" }
                        td { "/" }
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
                                    class: "btn btn-sm btn-warning",
                                    title: "Copy from this template",
                                    onclick: move |_| {
                                        let dialog_state = use_shared_state::<DialogState>(cx).unwrap();
                                        dialog_state
                                            .write()
                                            .show_dialog(
                                                "Edit template".to_string(),
                                                DialogType::AddTemplateFromOtherTemplate {
                                                    env: copy_env.to_string(),
                                                    name: copy_name.to_string(),
                                                },
                                            );
                                    },
                                    copy_from_icon {}
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
                        th {}
                        th { "Env" }
                        th {}
                        th {
                            table {
                                tr {
                                    td { "Name" }
                                    td { style: "width:100%",
                                        div { class: "input-group",
                                            span { class: "input-group-text", search_icon {} }
                                            input {
                                                class: "form-control form-control-sm",
                                                oninput: move |cx| {
                                                    filter.set(cx.value.to_string());
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
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
            let main_state = main_state.to_owned();
            cx.spawn(async move {
                let response = load_templates().await.unwrap();
                main_state.write().set_templates(Some(response));
            });

            render! { h1 { "loading" } }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateApiModel {
    pub env: String,
    pub name: String,
    pub created: String,
    pub updated: String,
    pub last_requests: i64,
    pub has_missing_placeholders: bool,
}

#[server]
async fn load_templates() -> Result<Vec<TemplateApiModel>, ServerFnError> {
    let response = crate::grpc_client::TemplatesGrpcClient::get_all_templates()
        .await
        .unwrap();

    let result: Vec<_> = response
        .into_iter()
        .map(|itm| TemplateApiModel {
            env: itm.env,
            name: itm.name,
            created: itm.created,
            updated: itm.updated,
            last_requests: itm.last_requests,
            has_missing_placeholders: itm.has_missing_placeholders,
        })
        .collect();

    Ok(result)

    /*
    let main_state = main_state.to_owned();

    cx.spawn(async move {


        main_state.write().set_templates(Some(response));
    });
     */
}

fn get_last_edited(templates: &Vec<TemplateApiModel>) -> (String, String) {
    let mut max = "";

    let mut env = "".to_string();
    let mut name = "".to_string();

    for template in templates {
        if template.updated.len() > 0 {
            if template.updated.as_str() > max {
                max = &template.updated;
                env = template.env.clone();
                name = template.name.clone();
            }
        }
    }

    (env, name)
}
