use std::rc::Rc;

use dioxus::prelude::*;

use dioxus_utils::*;

use crate::{dialogs::*, icons::*, models::*, states::*, ui_utils::ToastType};

use super::state::*;

#[component]
pub fn SecretsPage() -> Element {
    let ms = consume_context::<Signal<MainState>>();

    let ms_ra = ms.read();

    let selected_env_id = ms_ra.get_selected_env();

    let mut cs = use_signal(|| SecretsListState::new(&ms_ra));

    let cs_ra = cs.read();

    let mut filter_secret = consume_context::<Signal<FilterSecret>>();

    let filter_secret_read_access = filter_secret.read();

    let secrets = match get_data(ms, &ms_ra, selected_env_id.as_str(), &cs_ra.product_id) {
        Ok(secrets) => secrets,
        Err(err) => {
            return err;
        }
    };

    let last_edited = get_last_edited(secrets);

    let secrets = secrets
        .into_iter()
        .filter(|itm| filter_secret_read_access.filter(itm))
        .map(|itm| {
            let secret_id = Rc::new(itm.secret_id.to_string());

            let item_product_id = itm.product_id.clone().map(Rc::new);

            let secret3 = secret_id.clone();
            let edit_button_secret_name = secret_id.clone();
            let delete_secret_button = secret_id.clone();

            let mut class_template = "badge badge-success empty-links";
            let mut class_secret = class_template;

            let env_id_add = selected_env_id.clone();
            let env_id_delete = selected_env_id.clone();

            let env_id_usage = selected_env_id.clone();
            let env_id_usage_by_secret = selected_env_id.clone();

            if itm.used_by_templates == 0 && itm.used_by_secrets == 0 {
                class_template = "badge badge-danger have-no-links";
                class_secret = class_template;
            } else {
                if itm.used_by_templates > 0 {
                    class_template = "badge badge-success have-links";
                }

                if itm.used_by_secrets > 0 {
                    class_secret = "badge badge-success have-links";
                }
            };

            let secret_amount = itm.used_by_secrets;
            let templates_amount = itm.used_by_templates;

            let last_edited = if itm.secret_id.as_str() == last_edited.as_str() {
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

            let created = crate::utils::unix_microseconds_to_string(itm.created);
            let updated = crate::utils::unix_microseconds_to_string(itm.updated);

            let view_template_secret_id = secret_id.clone();
            let view_template_product_id = item_product_id.clone();

            let view_template_btn = rsx! {
                button {
                    class: "btn btn-sm btn-success",
                    onclick: move |_| {
                        let env_id = env_id_usage_by_secret.clone();
                        let secret_id = view_template_secret_id.clone();
                        let product_id = view_template_product_id.clone();
                        consume_context::<Signal<DialogState>>()
                            .set(DialogState::ShowSecret {
                                env_id,
                                secret_id,
                                product_id,
                            });
                    },
                    {view_template_icon()}
                }
            };

            let edit_product_id = item_product_id.clone();

            let edit_btn = rsx! {
                button {
                    class: "btn btn-sm btn-primary",
                    onclick: move |_| {
                        let secret_id = edit_button_secret_name.clone();
                        let env_id = env_id_add.clone();
                        let product_id = edit_product_id.clone();
                        consume_context::<Signal<DialogState>>()
                            .set(DialogState::EditSecret {
                                env_id: env_id.clone(),
                                product_id,
                                secret_id,
                                on_ok: EventHandler::new(move |value| {
                                    exec_save_secret(env_id.to_string(), value);
                                }),
                            })
                    },
                    EditIcon {}
                }
            };

            let delete_secret_product_id = item_product_id.clone();

            let delete_btn = rsx! {
                button {
                    class: "btn btn-sm btn-danger",
                    onclick: move |_| {
                        let secret_id = delete_secret_button.clone();
                        let product_id = delete_secret_product_id.clone();
                        let env_id = env_id_delete.clone();
                        consume_context::<Signal<DialogState>>()
                            .set(DialogState::Confirmation {
                                content: format!("Delete secret {}", delete_secret_button.as_str()),
                                on_ok: EventHandler::new(move |_| {
                                    exec_delete_secret(
                                        env_id.to_string(),
                                        product_id.as_ref().map(|itm| itm.to_string()),
                                        secret_id.to_string(),
                                    );
                                }),
                            });
                    },
                    DeleteIcon {}
                }
            };

            let copy_to_env_selected_env_id = selected_env_id.clone();
            let copy_to_env_secret_id = secret_id.clone();
            let copy_to_env_product_id = item_product_id.clone();

            let copy_to_env = rsx! {
                button {
                    class: "btn btn-sm btn-danger",
                    onclick: move |_| {
                        let from_env_id = copy_to_env_selected_env_id.clone();
                        let secret_id = copy_to_env_secret_id.clone();
                        let product_id = copy_to_env_product_id.clone();
                        consume_context::<Signal<DialogState>>()
                            .set(DialogState::CopyToEnvConfirmation {
                                from_env_id: from_env_id.clone(),
                                on_ok: EventHandler::new(move |env_id: String| {
                                    let from_env_id = from_env_id.clone();
                                    let secret_id = secret_id.clone();
                                    let product_id = product_id.clone();
                                    spawn(async move {
                                        crate::api::secrets::copy_secret_to_other_env(
                                                from_env_id.to_string(),
                                                env_id.to_string(),
                                                product_id.map(|itm| itm.to_string()),
                                                secret_id.to_string(),
                                            )
                                            .await
                                            .unwrap();
                                        crate::ui_utils::show_toast(
                                            format!("Secret has a copy at env {}", env_id.as_str()),
                                            ToastType::Info,
                                        );
                                    });
                                }),
                            });
                    },
                    CopyFromIcon {}
                }
            };

            let env_id_show_secret = selected_env_id.clone();
            let usage_product_id = item_product_id.clone();
            let usage_secret_id = secret_id.clone();

            let product_scope = match itm.product_id.as_ref() {
                Some(product_id) => rsx! {
                    span { class: "badge text-bg-light", {product_id.as_str()} }
                },
                None => rsx! {
                    span { class: "badge text-bg-warning", "Shared" }
                },
            };
            rsx! {
                tr { style: "border-top: 1px solid lightgray;",
                    td { style: "padding-left: 10px",
                        div { style: "padding: 0;",
                            span {
                                class: "{class_template}",
                                onclick: move |_| {
                                    if templates_amount == 0 {
                                        return;
                                    }
                                    let env_id = env_id_show_secret.clone();
                                    let secret_name = usage_secret_id.clone();
                                    let product_id = usage_product_id.clone();
                                    consume_context::<Signal<DialogState>>()
                                        .set(DialogState::SecretUsage {
                                            env_id,
                                            product_id,
                                            secret_id: secret_name,
                                        })
                                },
                                "{templates_amount}"
                            }
                        }
                    }

                    td {
                        div { style: "padding: 0;",
                            span {
                                class: "{class_secret}",
                                onclick: move |_| {
                                    if secret_amount == 0 {
                                        return;
                                    }
                                    let env_id = env_id_usage.clone();
                                    let secret_id = secret3.clone();
                                    consume_context::<Signal<DialogState>>()
                                        .set(DialogState::SecretUsageBySecret {
                                            env_id,
                                            secret_id,
                                        });
                                },
                                "{itm.used_by_secrets}"
                            }
                        }
                    }
                    td { {product_scope} }
                    td { style: "padding: 10px",
                        "{itm.secret_id}"
                        {last_edited}
                    }
                    td { "{itm.level}" }
                    td { "{created.without_microseconds()}" }
                    td { "{updated.without_microseconds()}" }
                    td {
                        div { class: "btn-group",
                            {copy_to_env}
                            {view_template_btn}
                            {edit_btn}
                            {delete_btn}
                        }
                    }
                }
            }
        });

    let edit_secret_product_id = cs_ra.product_id.clone();
    rsx! {
        table { class: "table table-striped", style: "text-align: left;",
            thead {
                tr {
                    th { style: "padding: 10px", colspan: "2", "Used" }
                    th { "Product scope" }
                    th { style: "width:50%",
                        table {
                            tr {
                                td {
                                    style: "cursor:pointer",
                                    onclick: move |_| {
                                        cs.write().order_by = OrderBy::Name;
                                    },
                                    "Name"
                                }
                                td { style: "width:90%",
                                    div { class: "input-group",
                                        span { class: "input-group-text", SearchIcon {} }
                                        input {
                                            class: "form-control form-control-sm",
                                            value: filter_secret_read_access.as_str(),
                                            oninput: move |cx| {
                                                let mut write = filter_secret.write();
                                                write.set_value(cx.value().as_str());
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }
                    th { "Level" }
                    th { "Created" }
                    th {
                        style: "cursor:pointer",
                        onclick: move |_| {
                            cs.write().order_by = OrderBy::Updated;
                        },
                        "Updated"
                    }
                    th {
                        div {
                            button {
                                class: "btn btn-sm btn-primary",
                                onclick: move |_| {
                                    let env_id = selected_env_id.clone();
                                    let product_id = Some(edit_secret_product_id.clone());
                                    consume_context::<Signal<DialogState>>()
                                        .set(DialogState::EditSecret {
                                            env_id: env_id.clone(),
                                            product_id,
                                            secret_id: "".to_string().into(),
                                            on_ok: EventHandler::new(move |value| {
                                                exec_save_secret(env_id.to_string(), value);
                                            }),
                                        })
                                },
                                AddIcon {}
                            }
                        }
                    }
                }
            }
            tbody { {secrets.into_iter()} }
        }
    }
}

fn get_data<'s>(
    mut ms: Signal<MainState>,
    ms_ra: &'s MainState,
    env_id: &str,
    product_id: &str,
) -> Result<&'s [SecretHttpModel], Element> {
    match ms_ra.secrets.as_ref() {
        RenderState::None => {
            let env_id = env_id.to_string();
            let product_id = product_id.to_string();

            spawn(async move {
                ms.write().secrets.set_loading();
                match crate::api::secrets::load_secrets(env_id, product_id).await {
                    Ok(value) => {
                        ms.write().secrets.set_value(value);
                    }
                    Err(err) => {
                        ms.write().secrets.set_error(err.to_string());
                    }
                }
            });
            return Err(crate::icons::loading_icon());
        }
        RenderState::Loading => {
            return Err(crate::icons::loading_icon());
        }
        RenderState::Loaded(value) => Ok(value.as_slice()),
        RenderState::Error(err) => {
            return Err(crate::icons::render_error(err));
        }
    }
}

fn exec_save_secret(env_id: String, value: UpdateSecretValueHttpModel) {
    if let Some(product_id) = value.product_id.as_ref() {
        crate::storage::last_used_product::save(product_id);
    }
    spawn(async move {
        match crate::api::secrets::save_secret(env_id, value).await {
            Ok(_) => {
                consume_context::<Signal<MainState>>().write().drop_data();
                crate::ui_utils::show_toast("Secret is saved", ToastType::Info);
            }
            Err(_) => {
                crate::ui_utils::show_toast("Error saving secret", ToastType::Error);
            }
        }
    });
}

fn exec_delete_secret(env_id: String, product_id: Option<String>, secret_id: String) {
    spawn(async move {
        match crate::api::secrets::delete_secret(env_id, product_id, secret_id).await {
            Ok(_) => {
                consume_context::<Signal<MainState>>().write().drop_data();
                crate::ui_utils::show_toast("Secret is deleted", ToastType::Info);
            }
            Err(_) => {
                crate::ui_utils::show_toast("Error deleting secret", ToastType::Error);
            }
        }
    });
}

fn get_last_edited(secrets: &[SecretHttpModel]) -> String {
    let mut max = 0;

    let mut value = "".to_string();

    for secret in secrets {
        if secret.updated > 0 {
            if secret.updated > max {
                max = secret.updated;
                value = secret.secret_id.clone();
            }
        }
    }

    value
}
