use std::rc::Rc;

use super::icons::*;
use crate::{states::*, ui_utils::ToastType};
use dioxus::prelude::*;

use crate::dialogs::*;
use dioxus_utils::DataState;
use serde::*;

#[component]
pub fn TemplatesList() -> Element {
    let mut main_state = consume_context::<Signal<MainState>>();

    let main_state_read_access = main_state.read();

    let env_id = main_state_read_access.get_selected_env();

    let mut filter_template = consume_context::<Signal<FilterTemplate>>();
    let filter_template_read_access = filter_template.read();

    let templates = match &main_state_read_access.templates {
        dioxus_utils::DataState::None => {
            let env_id_request = env_id.clone();
            spawn(async move {
                main_state.write().templates = dioxus_utils::DataState::Loading;
                match load_templates(env_id_request.to_string()).await {
                    Ok(templates) => {
                        main_state.write().templates = dioxus_utils::DataState::Loaded(templates);
                    }
                    Err(err) => {
                        main_state.write().templates =
                            dioxus_utils::DataState::Error(err.to_string());
                    }
                }
            });

            return rsx! {
                LoadingIcon {}
            };
        }
        DataState::Loading => {
            return rsx! {
                LoadingIcon {}
            }
        }
        DataState::Loaded(result) => result,
        DataState::Error(err) => {
            return rsx! {
                {err.as_str()}
            }
        }
    };

    let last_edited = get_last_edited(templates);
    let templates = templates
        .iter()
        .filter(|itm| filter_template_read_access.filter_record(itm))
        .map(|itm| {
            let last_request = if itm.last_requests == 0 {
                "".to_string()
            } else {
                crate::utils::unix_microseconds_to_string(itm.last_requests * 1000)
                    .without_microseconds()
                    .to_string()
            };

            let env = Rc::new(itm.env.to_string());
            let name = Rc::new(itm.name.to_string());

            let show_populated_yaml_env = env.clone();
            let show_populated_yaml_name = name.clone();

            let delete_template_env = env.clone();
            let delete_template_name = name.clone();

            let init_env = env.clone();
            let init_name = name.clone();

            let env_id_edit = env_id.clone();
            let env_id_copy = env_id.clone();
            let env_id_delete = env_id.clone();
            let env_id_show_populated_yaml = env_id.clone();

            let last_edited = if last_edited.0.as_str() == env.as_str()
                && last_edited.1.as_str() == name.as_str()
            {
                Some(rsx!(
                    span {
                        id: "last-edited-badge",
                        class: "badge badge-success not-selectable",
                        "Last edited"
                    }
                    script { r#"scroll_to('last-edited-badge')"# }
                ))
            } else {
                None
            };

            let alert = if itm.has_missing_placeholders {
                Some(rsx! {
                    div { WarningIcon {} }
                })
            } else {
                None
            };

            let created = crate::utils::unix_microseconds_to_string(itm.created);
            let updated = crate::utils::unix_microseconds_to_string(itm.updated);

            rsx! {
                tr { style: "border-top: 1px solid lightgray",
                    td { {alert} }
                    td { "{itm.env}" }
                    td { "/" }
                    td {
                        "{itm.name}"
                        {last_edited}
                    }
                    td { {created.without_microseconds()} }
                    td { {updated.without_microseconds()} }
                    td { "{last_request}" }
                    td {
                        div { class: "btn-group",
                            button {
                                class: "btn btn-sm btn-success",
                                onclick: move |_| {
                                    let env_id = env_id_show_populated_yaml.clone();
                                    let env = show_populated_yaml_env.clone();
                                    let name = show_populated_yaml_name.clone();
                                    consume_context::<Signal<DialogState>>()
                                        .set(DialogState::ShowPopulatedYaml {
                                            env_id,
                                            env,
                                            name,
                                        });
                                },
                                ViewTemplateIcon {}
                            }
                            button {
                                class: "btn btn-sm btn-primary",
                                onclick: move |_| {
                                    let env_id = env_id_edit.clone();
                                    let env = env.clone();
                                    let name = name.clone();
                                    consume_context::<Signal<DialogState>>()
                                        .set(DialogState::EditTemplate {
                                            env_id: env_id.clone(),
                                            env,
                                            name,
                                            init_from_other_template: None,
                                            on_ok: EventHandler::new(move |result| {
                                                exec_save_template(env_id.to_string(), result);
                                            }),
                                        });
                                },
                                EditIcon {}
                            }
                            button {
                                class: "btn btn-sm btn-warning",
                                title: "Copy from this template",
                                onclick: move |_| {
                                    let env_id = env_id_copy.clone();
                                    let init_env = init_env.clone();
                                    let init_name = init_name.clone();
                                    consume_context::<Signal<DialogState>>()
                                        .set(DialogState::EditTemplate {
                                            env_id: env_id.clone(),
                                            env: String::new().into(),
                                            name: String::new().into(),
                                            init_from_other_template: Some((init_env, init_name)),
                                            on_ok: EventHandler::new(move |result| {
                                                exec_save_template(env_id.to_string(), result);
                                            }),
                                        });
                                },
                                CopyFromIcon {}
                            }
                            button {
                                class: "btn btn-sm btn-danger",
                                onclick: move |_| {
                                    let env_id = env_id_delete.clone();
                                    let env = delete_template_env.clone();
                                    let name = delete_template_name.clone();
                                    consume_context::<Signal<DialogState>>()
                                        .set(DialogState::Confirmation {
                                            content: format!(
                                                "Please confirm deletion of template {}/{}",
                                                delete_template_env.as_str(),
                                                delete_template_name.as_str(),
                                            ),
                                            on_ok: EventHandler::new(move |_| {
                                                exec_delete_template(
                                                    env_id.to_string(),
                                                    env.to_string(),
                                                    name.to_string(),
                                                );
                                            }),
                                        })
                                },
                                DeleteIcon {}
                            }
                        }
                    }
                }
            }
        });

    rsx! {
        table { class: "table table-striped", style: "text-align: left;",
            thead {
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
                                        span { class: "input-group-text", SearchIcon {} }
                                        input {
                                            class: "form-control form-control-sm",
                                            value: filter_template_read_access.as_str(),
                                            oninput: move |cx| {
                                                filter_template.write().set_value(cx.value().as_str());
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
                                onclick: move |_| {
                                    let env_id = env_id.clone();
                                    consume_context::<Signal<DialogState>>()
                                        .set(DialogState::EditTemplate {
                                            env_id: env_id.clone(),
                                            env: String::new().into(),
                                            name: String::new().into(),
                                            init_from_other_template: None,
                                            on_ok: EventHandler::new(move |result| {
                                                exec_save_template(env_id.to_string(), result);
                                            }),
                                        });
                                },
                                AddIcon {}
                            }
                        }
                    }
                }
            }

            tbody { {templates.into_iter()} }
        }
    }
}

fn exec_save_template(env_id: String, save_template_result: SaveTemplateResult) {
    spawn(async move {
        match save_template(
            env_id,
            save_template_result.env,
            save_template_result.name,
            save_template_result.yaml,
        )
        .await
        {
            Ok(_) => {
                consume_context::<Signal<DialogState>>().set(DialogState::None);
                consume_context::<Signal<MainState>>().write().drop_data();
                crate::ui_utils::show_toast("Template is saved", ToastType::Info);
            }
            Err(_) => {
                crate::ui_utils::show_toast("Error saving templated", ToastType::Error);
            }
        }
    });
}

fn exec_delete_template(env_id: String, env: String, name: String) {
    spawn(async move {
        match delete_template(env_id, env, name).await {
            Ok(_) => {
                consume_context::<Signal<DialogState>>().set(DialogState::None);
                consume_context::<Signal<MainState>>().write().drop_data();
                crate::ui_utils::show_toast("Template is deleted", ToastType::Info);
            }
            Err(_) => {
                crate::ui_utils::show_toast("Error deleting templated", ToastType::Error);
            }
        }
    });
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateApiModel {
    pub env: String,
    pub name: String,
    pub created: i64,
    pub updated: i64,
    pub last_requests: i64,
    pub has_missing_placeholders: bool,
}

#[server]
async fn load_templates(env_id: String) -> Result<Vec<TemplateApiModel>, ServerFnError> {
    use rust_extensions::date_time::DateTimeAsMicroseconds;
    use std::collections::BTreeMap;

    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    let response = ctx
        .templates_grpc
        .get_all(())
        .await
        .unwrap()
        .unwrap_or_default();

    let result: BTreeMap<_, _> = response
        .into_iter()
        .map(|itm| {
            (
                format!("{}/{}", itm.name, itm.env),
                TemplateApiModel {
                    env: itm.env,
                    name: itm.name,
                    created: match DateTimeAsMicroseconds::from_str(itm.created.as_str()) {
                        Some(itm) => itm.unix_microseconds,
                        None => 0,
                    },
                    updated: match DateTimeAsMicroseconds::from_str(itm.updated.as_str()) {
                        Some(itm) => itm.unix_microseconds,
                        None => 0,
                    },
                    last_requests: itm.last_requests,
                    has_missing_placeholders: itm.has_missing_placeholders,
                },
            )
        })
        .collect();

    let result = result.into_iter().map(|itm| itm.1).collect();

    Ok(result)
}

fn get_last_edited(templates: &Vec<TemplateApiModel>) -> (String, String) {
    let mut max = 0;

    let mut env = "".to_string();
    let mut name = "".to_string();

    for template in templates {
        if template.updated > 0 {
            if template.updated > max {
                max = template.updated;
                env = template.env.clone();
                name = template.name.clone();
            }
        }
    }

    (env, name)
}

#[server]
pub async fn save_template(
    env_id: String,
    env: String,
    name: String,
    yaml: String,
) -> Result<(), ServerFnError> {
    use crate::server::templates_grpc::*;
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    ctx.templates_grpc
        .save(SaveTemplateRequest { env, name, yaml })
        .await
        .unwrap();

    Ok(())
}

#[server]
async fn delete_template(env_id: String, env: String, name: String) -> Result<(), ServerFnError> {
    use crate::server::templates_grpc::*;
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    ctx.templates_grpc
        .delete(DeleteTemplateRequest { env, name })
        .await
        .unwrap();

    Ok(())
}
