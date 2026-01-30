use std::rc::Rc;

use crate::dialogs::states::EditTemplateDialogData;
use crate::icons::*;
use crate::models::*;
use crate::utils::DateTimeRfc3339;
use crate::{states::*, ui_utils::ToastType};
use dioxus::prelude::*;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use rust_extensions::duration_utils::DurationExtensions;

use crate::dialogs::*;
use dioxus_utils::*;

use super::state::*;

#[component]
pub fn TemplatesPage() -> Element {

    let now = dioxus_utils::now_date_time();
     let selected_env = Rc::new(crate::storage::selected_env::get());
    let mut cs = use_signal(|| TemplatesState::new(selected_env.as_str()));
    let cs_ra = cs.read();

    let ms = consume_context::<Signal<MainState>>();
    let ms_ra = ms.read();

   

    let templates = match get_data(&ms_ra, &selected_env) {
        Ok(items) => items,
        Err(err) => return err,
    };

    let selected_env_id_to_copy = selected_env.clone();

    let last_edited = get_last_edited(templates);
    let templates = templates
        .iter()
        .filter(|itm| cs_ra.filter_record(itm))
        .map(|itm| {
            let (last_request, last_duration) = if itm.last_requests == 0 {
                (String::new(), String::new())
            } else {

                let dt = DateTimeAsMicroseconds::from(itm.last_requests); 

                let duration = now.duration_since(dt);
                let as_string = DateTimeRfc3339::from_dt(dt);
                let dt = as_string
                    .without_microseconds()
                    .to_string();


                    (dt, duration.as_positive_or_zero().format_to_string())
            };

            let template_to_copy = itm.clone();
            let template_to_edit = itm.clone();

            let product_id = Rc::new(itm.product_id.to_string());
            let template_id = Rc::new(itm.template_id.to_string());

            let show_populated_yaml_product_id = product_id.clone();
            let show_populated_yaml_template_id = template_id.clone();

            let delete_product_id = product_id.clone();
            let delete_template_id = template_id.clone();

            let env_id_edit = selected_env_id_to_copy.clone();
            let env_id_copy = selected_env_id_to_copy.clone();
            let delete_env_id = selected_env_id_to_copy.clone();
            let env_id_show_populated_yaml = selected_env_id_to_copy.clone();

            let last_edited = if last_edited.0.as_str() == product_id.as_str()
                && last_edited.1.as_str() == template_id.as_str()
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
                                    exec_save_template(cs, env_id.to_string(), result);
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
                                    exec_save_template(cs, env_id.to_string(), result);
                                }),
                            });
                    },
                    EditIcon {}
                }
            };

            let copy_to_env_selected_env_id = selected_env.clone();
            let copy_to_env_template_product_id = product_id.clone();
            let copy_to_env_template_template_id = template_id.clone();

            let copy_to_env = rsx! {
                button {
                    class: "btn btn-sm btn-danger",
                    onclick: move |_| {
                        let from_env_id = copy_to_env_selected_env_id.clone();
                        let product_id = copy_to_env_template_product_id.clone();
                        let template_id = copy_to_env_template_template_id.clone();
                        consume_context::<Signal<DialogState>>()
                            .set(DialogState::CopyToEnvConfirmation {
                                from_env_id: from_env_id.clone(),
                                on_ok: EventHandler::new(move |env_id: String| {
                                    let from_env_id = from_env_id.clone();
                                    let product_id = product_id.clone();
                                    let template_id = template_id.clone();
                                    spawn(async move {
                                        crate::api::templates::copy_template_to_other_env(
                                                from_env_id.to_string(),
                                                env_id.to_string(),
                                                product_id.to_string(),
                                                template_id.to_string(),
                                            )
                                            .await
                                            .unwrap();
                                        crate::ui_utils::show_toast(
                                            format!("Template has a copy at env {}", env_id.as_str()),
                                            ToastType::Info,
                                        );
                                    });
                                }),
                            });
                    },
                    CopyFromIcon {}
                }
            };

            let selected = cs_ra.is_selected(&product_id.as_str(), template_id.as_str());

            let selected = crate::icons::render_bool_checkbox(selected, EventHandler::new(move |value|{
                cs.write().set_selected(product_id.as_str(), template_id.as_str(), value);
            }));


            rsx! {
                tr { style: "border-top: 1px solid lightgray",
                    td { {alert} }
                    td { {selected} }
                    td { "{itm.product_id}" }
                    td { "/" }
                    td {
                        "{itm.template_id}"
                        {last_edited}
                    }
                    td { {created.without_microseconds()} }
                    td { {updated.without_microseconds()} }
                    td {
                        div { style: "textsize:9px", "{last_request}" }
                        div { "{last_duration}" }
                    }
                    td {
                        div { class: "btn-group",
                            {copy_to_env}
                            button {
                                class: "btn btn-sm btn-success",
                                onclick: move |_| {
                                    let env_id = env_id_show_populated_yaml.clone();
                                    let product_id = show_populated_yaml_product_id.clone();
                                    let template_id = show_populated_yaml_template_id.clone();
                                    consume_context::<Signal<DialogState>>()
                                        .set(DialogState::ShowPopulatedYaml {
                                            env_id,
                                            product_id,
                                            template_id,
                                        });
                                },
                                {view_template_icon()}
                            }

                            {copy_from_template_btn}
                            {edit_btn}
                            button {
                                class: "btn btn-sm btn-danger",
                                onclick: move |_| {

                                    let env_id = delete_env_id.clone();
                                    let product_id = delete_product_id.clone();
                                    let template_id = delete_template_id.clone();

                                    consume_context::<Signal<DialogState>>()
                                        .set(DialogState::Confirmation {
                                            content: format!(
                                                "Please confirm deletion of template {}/{}",
                                                product_id.as_str(),
                                                template_id.as_str(),
                                            ),
                                            on_ok: EventHandler::new(move |_| {
                                                exec_delete_template(
                                                    env_id.to_string(),
                                                    product_id.to_string(),
                                                    template_id.to_string(),
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

    let selected_env_spawned = selected_env.clone();

    let add_btn = rsx! {
        button {
            class: "btn btn-sm btn-primary",
            onclick: move |_| {
                let env_id = selected_env_spawned.clone();
                consume_context::<Signal<DialogState>>()
                    .set(DialogState::EditTemplate {
                        env_id: env_id.clone(),
                        data: EditTemplateDialogData::New,
                        on_ok: EventHandler::new(move |result| {
                            exec_save_template(cs, env_id.to_string(), result);
                        }),
                    });
            },
            AddIcon {}
        }
    };

    let selected_env_id = selected_env.clone();

    let export_btn = if cs_ra.has_selected() {
        rsx! {
            button {
                class: "btn btn-sm btn-primary",
                onclick: move |_| {
                    let selected_env_id = selected_env_id.clone();
                    spawn(async move {
                        let request = cs.read().get_request_data();
                        let yaml = crate::api::templates::download_snapshot(
                                selected_env_id.to_string(),
                                request,
                            )
                            .await
                            .unwrap();
                        consume_context::<Signal<DialogState>>()
                            .set(DialogState::SnapshotToExport(Rc::new(yaml)));
                    });
                },
                "Export"
            }
        }
    } else {
        rsx! {}
    };

    let selected_env_id = selected_env.clone();
    let import_btn = rsx! {
        button {
            class: "btn btn-sm btn-primary",
            onclick: move |_| {
                let env_id = selected_env_id.clone();
                consume_context::<Signal<DialogState>>()
                    .set(
                        DialogState::SnapshotToImport(
                            EventHandler::new(move |value| {
                                let env_id = env_id.clone();
                                spawn(async move {
                                    crate::api::templates::upload_snapshot(
                                            env_id.to_string(),
                                            value,
                                        )
                                        .await
                                        .unwrap();
                                    consume_context::<Signal<MainState>>().write().drop_data();
                                    crate::ui_utils::show_toast(
                                        "Templates are uploaded",
                                        ToastType::Info,
                                    );
                                });
                            }),
                        ),
                    );
            },
            "Import"
        }
    };


    let select_product_env_id = selected_env.clone();
    let select_product = crate::components::select_product(
        &ms_ra,
        None,
        cs_ra.product_id.as_deref(),
        false,
        EventHandler::new(move |value: Option<String>| {
            if let Some(value) = value.as_ref() {
                crate::storage::last_used_product::save(select_product_env_id.as_str(), value);
            }
            cs.write().product_id = value;
        }),
    );

    rsx! {
        table { class: "table table-striped", style: "text-align: left;",
            thead {
                tr {
                    th {}
                    th { {export_btn} }
                    th {
                        "Product"
                        {select_product}
                    }
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
                                            value: cs_ra.filter.as_str(),
                                            oninput: move |cx| {
                                                cs.write().filter = cx.value();
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
                    th {
                        {add_btn}
                        {import_btn}
                    }
                }
            }

            tbody { {templates.into_iter()} }
        }
    }
}

fn exec_save_template(
    mut cs: Signal<TemplatesState>,
    env_id: String,
    data: UpdateTemplateHttpModel,
) {
    crate::storage::last_used_product::save(env_id.as_str(), &data.product_id);

    spawn(async move {
        let product_id = data.product_id.clone();
        match crate::api::templates::save_template(env_id, data).await {
            Ok(_) => {
                consume_context::<Signal<DialogState>>().set(DialogState::None);
                consume_context::<Signal<MainState>>().write().drop_data();
                cs.write().product_id = Some(product_id);
                crate::ui_utils::show_toast("Template is saved", ToastType::Info);
            }
            Err(_) => {
                crate::ui_utils::show_toast("Error saving templated", ToastType::Error);
            }
        }
    });
}

fn exec_delete_template(env_id: String, product_id: String, template_id: String) {
    spawn(async move {
        match crate::api::templates::delete_template(env_id, product_id, template_id).await {
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

fn get_last_edited(templates: &[Rc<TemplateHttpModel>]) -> (String, String) {
    let mut max = 0;

    let mut product_id = "".to_string();
    let mut template_id = "".to_string();

    for template in templates {
        if template.updated > 0 {
            if template.updated > max {
                max = template.updated;
                product_id = template.product_id.clone();
                template_id = template.template_id.clone();
            }
        }
    }

    (product_id, template_id)
}

fn get_data<'s>(
    ms_ra: &'s MainState,
    selected_env: &str,
) -> Result<&'s [Rc<TemplateHttpModel>], Element> {
    match ms_ra.templates.as_ref() {
        RenderState::None => {
            let env_id_request = selected_env.to_string();
            spawn(async move {
                let mut ms = consume_context::<Signal<MainState>>();
                ms.write().templates.set_loading();
                match crate::api::templates::get_templates(env_id_request).await {
                    Ok(templates) => {
                        ms.write().set_templates_as_loaded(templates);
                    }
                    Err(err) => {
                        ms.write().templates.set_error(err.to_string());
                    }
                }
            });

            return Err(crate::icons::loading_icon());
        }
        RenderState::Loading => {
            return Err(crate::icons::loading_icon());
        }
        RenderState::Loaded(result) => {
            return Ok(result.as_slice());
        }
        RenderState::Error(err) => {
            return Err(crate::icons::render_error(err));
        }
    }
}
