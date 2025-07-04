use std::rc::Rc;

use crate::dialogs::states::EditTemplateDialogData;
use crate::icons::*;
use crate::models::*;
use crate::{states::*, ui_utils::ToastType};
use dioxus::prelude::*;

use crate::dialogs::*;
use dioxus_utils::DataState;

use super::state::*;

#[component]
pub fn TemplatesPage() -> Element {
    let mut cs = use_signal(|| TemplatesState::default());

    let cs_ra = cs.read();
    let mut main_state = consume_context::<Signal<MainState>>();

    let main_state_read_access = main_state.read();

    let env_id = main_state_read_access.get_selected_env();

    let env_id_to_copy = env_id.clone();

    let mut filter_template = consume_context::<Signal<FilterTemplate>>();
    let filter_template_read_access = filter_template.read();

    let templates = match &main_state_read_access.templates {
        dioxus_utils::DataState::None => {
            let env_id_request = env_id.clone();
            spawn(async move {
                main_state.write().templates = dioxus_utils::DataState::Loading;
                match crate::api::templates::get_templates(env_id_request.to_string()).await {
                    Ok(templates) => {
                        main_state
                            .write()
                            .templates
                            .set_loaded(templates.into_iter().map(Rc::new).collect());
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

            let template_to_copy = itm.clone();
            let template_to_edit = itm.clone();

            let env = Rc::new(itm.env.to_string());
            let name = Rc::new(itm.name.to_string());

            let show_populated_yaml_env = env.clone();
            let show_populated_yaml_name = name.clone();

            let delete_template_env = env.clone();
            let delete_template_name = name.clone();

            let env_id_edit = env_id_to_copy.clone();
            let env_id_copy = env_id_to_copy.clone();
            let env_id_delete = env_id_to_copy.clone();
            let env_id_show_populated_yaml = env_id_to_copy.clone();

            let env_id_select = env_id_to_copy.clone();

            let name_select = name.clone();

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

            let copy_from_template_btn = rsx! {
                button {
                    class: "btn btn-sm btn-warning",
                    title: "Copy from this template",
                    onclick: move |_| {
                        let env_id = env_id_copy.clone();
                        let template_to_copy = template_to_copy.clone();
                        consume_context::<Signal<DialogState>>()
                            .set(DialogState::EditTemplate {
                                env_id: env_id.clone(),
                                data: EditTemplateDialogData::CopyFromOtherTemplate(template_to_copy),
                                on_ok: EventHandler::new(move |result| {
                                    exec_save_template(env_id.to_string(), result);
                                }),
                            });
                    },
                    CopyFromIcon {}
                }
            };

            let edit_btn = rsx!{
                button {
                    class: "btn btn-sm btn-primary",
                    onclick: move |_| {
                        let env_id = env_id_edit.clone();
                        let template_to_edit = template_to_edit.clone();
                        consume_context::<Signal<DialogState>>()
                            .set(DialogState::EditTemplate {
                                env_id: env_id.clone(),
                                data: EditTemplateDialogData::Edit(template_to_edit),
                                on_ok: EventHandler::new(move |result| {
                                    exec_save_template(env_id.to_string(), result);
                                }),
                            });
                    },
                    EditIcon {}
                }
            };

            let selected = cs_ra.is_selected(env_id_select.as_str(), name_select.as_str());

            let selected = crate::icons::render_bool_checkbox(selected, EventHandler::new(move |value|{
                cs.write().set_selected(env_id_select.as_str(), name_select.as_str(), value);
            }));


            rsx! {
                tr { style: "border-top: 1px solid lightgray",
                    td { {alert} }
                    td { {selected} }
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
                                {view_template_icon()}
                            }

                            {copy_from_template_btn}
                            {edit_btn}
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

    let add_btn = rsx! {
        button {
            class: "btn btn-sm btn-primary",
            onclick: move |_| {
                let env_id = env_id.clone();
                consume_context::<Signal<DialogState>>()
                    .set(DialogState::EditTemplate {
                        env_id: env_id.clone(),
                        data: EditTemplateDialogData::New,
                        on_ok: EventHandler::new(move |result| {
                            exec_save_template(env_id.to_string(), result);
                        }),
                    });
            },
            AddIcon {}
        }
    };

    let export_btn = if cs_ra.has_selected() {
        rsx! {
            button {
                class: "btn btn-sm btn-primary",
                onclick: move |_| { download_file() },
                "Export"
            }
        }
    } else {
        rsx! {}
    };

    rsx! {
        table { class: "table table-striped", style: "text-align: left;",
            thead {
                tr {
                    th { {export_btn} }
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
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }
                    th { "Created" }
                    th { "Updated" }
                    th { "Last request" }
                    th { {add_btn} }
                }
            }

            tbody { {templates.into_iter()} }
        }
    }
}

fn exec_save_template(env_id: String, data: UpdateTemplateHttpModel) {
    spawn(async move {
        match crate::api::templates::save_template(env_id, data).await {
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
        match crate::api::templates::delete_template(env_id, env, name).await {
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

fn get_last_edited(templates: &Vec<Rc<TemplateHttpModel>>) -> (String, String) {
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

fn download_file() {
    use rfd::FileDialog;
    // File content
    let content = "Hello, this is a sample file content!".to_string();

    // Open a file save dialog
    if let Some(path) = FileDialog::new()
        .set_file_name("sample.txt")
        .add_filter("Text", &["txt"])
        .save_file()
    {
        // Write the content to the selected file
        if let Err(e) = std::fs::write(&path, content) {
            eprintln!("Failed to write file: {}", e);
        } else {
            println!("File saved to {:?}", path);
        }
    }
}
