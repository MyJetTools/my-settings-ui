use std::{collections::BTreeMap, rc::Rc};

use dioxus::prelude::*;

use dioxus_utils::DataState;

use crate::{dialogs::*, icons::*, models::*, states::*, ui_utils::ToastType};

#[derive(Debug, Clone, Copy)]
pub enum OrderBy {
    Name,
    Updated,
}

#[component]
pub fn SecretsPage() -> Element {
    let mut main_state = consume_context::<Signal<MainState>>();

    let main_state_read_access = main_state.read();

    let selected_env_id = main_state_read_access.get_selected_env();

    let mut component_state = use_signal(|| SecretsListState::new());

    let component_state_read_access = component_state.read();

    let mut filter_secret = consume_context::<Signal<FilterSecret>>();

    let filter_secret_read_access = filter_secret.read();

    let secrets = match &main_state_read_access.secrets {
        DataState::None => {
            let env_id = selected_env_id.clone();
            spawn(async move {
                main_state.write().secrets = dioxus_utils::DataState::Loading;
                match crate::api::secrets::load_secrets(env_id.to_string()).await {
                    Ok(value) => {
                        main_state.write().secrets = dioxus_utils::DataState::Loaded(value);
                    }
                    Err(err) => {
                        main_state.write().secrets =
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
        DataState::Loaded(value) => value,
        DataState::Error(err) => {
            return rsx! {
                div { {err.as_str()} }
            }
        }
    };

    let last_edited = get_last_edited(&secrets);

    let sorted = component_state_read_access.sort(secrets.iter());

    let (name_up, updated_up) = match component_state_read_access.order_by {
        OrderBy::Name => ("▲", ""),
        OrderBy::Updated => ("", "▲"),
    };

    let secrets = sorted
        .into_iter()
        .filter(|itm| filter_secret_read_access.filter(&itm.1))
        .map(|(_, itm)| {
            let secret = Rc::new(itm.name.to_string());
            let secret_usage_name = secret.clone();
            let secret3 = secret.clone();
            let edit_button_secret_name = secret.clone();
            let delete_secret_button = secret.clone();

            let mut class_template = "badge badge-success empty-links";
            let mut class_secret = class_template;

            let env_id_add = selected_env_id.clone();
            let env_id_delete = selected_env_id.clone();
            let env_id_show_secret = selected_env_id.clone();
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

            let last_edited = if itm.name.as_str() == last_edited.as_str() {
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

            let view_template_secret = secret.clone();
            let view_template_btn = rsx! {
                button {
                    class: "btn btn-sm btn-success",
                    onclick: move |_| {
                        let env_id = env_id_usage_by_secret.clone();
                        let secret = view_template_secret.clone();
                        consume_context::<Signal<DialogState>>()
                            .set(DialogState::ShowSecret {
                                env_id,
                                secret,
                            });
                    },
                    {view_template_icon()}
                }
            };

            let edit_btn = rsx! {
                button {
                    class: "btn btn-sm btn-primary",
                    onclick: move |_| {
                        let name = edit_button_secret_name.clone();
                        let env_id = env_id_add.clone();
                        consume_context::<Signal<DialogState>>()
                            .set(DialogState::EditSecret {
                                env_id: env_id.clone(),
                                name,
                                on_ok: EventHandler::new(move |result: EditSecretResult| {
                                    exec_save_secret(env_id.to_string(), result);
                                }),
                            })
                    },
                    EditIcon {}
                }
            };

            let delete_btn = rsx! {
                button {
                    class: "btn btn-sm btn-danger",
                    onclick: move |_| {
                        let secret_id = delete_secret_button.clone();
                        let env_id = env_id_delete.clone();
                        consume_context::<Signal<DialogState>>()
                            .set(DialogState::Confirmation {
                                content: format!("Delete secret {}", delete_secret_button.as_str()),
                                on_ok: EventHandler::new(move |_| {
                                    exec_delete_secret(env_id.to_string(), secret_id.to_string());
                                }),
                            });
                    },
                    DeleteIcon {}
                }
            };

            let copy_to_env_selected_env_id = selected_env_id.clone();
            let copy_to_env_secret_id = secret.clone();

            let copy_to_env = rsx! {
                button {
                    class: "btn btn-sm btn-danger",
                    onclick: move |_| {
                        let from_env_id = copy_to_env_selected_env_id.clone();
                        let secret_id = copy_to_env_secret_id.clone();
                        consume_context::<Signal<DialogState>>()
                            .set(DialogState::CopyToEnvConfirmation {
                                from_env_id: from_env_id.clone(),
                                on_ok: EventHandler::new(move |env_id: String| {
                                    let from_env_id = from_env_id.clone();
                                    let secret_id = secret_id.clone();
                                    spawn(async move {
                                        crate::api::secrets::copy_secret_to_other_env(
                                                from_env_id.to_string(),
                                                env_id.to_string(),
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
                                    let secret_name = secret_usage_name.clone();
                                    consume_context::<Signal<DialogState>>()
                                        .set(DialogState::SecretUsage {
                                            env_id,
                                            secret: secret_name,
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
                                    let secret = secret3.clone();
                                    consume_context::<Signal<DialogState>>()
                                        .set(DialogState::SecretUsageBySecret {
                                            env_id,
                                            secret,
                                        });
                                },
                                "{itm.used_by_secrets}"
                            }
                        }
                    }
                    td { style: "padding: 10px",
                        "{itm.name}"
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

    rsx! {
        table { class: "table table-striped", style: "text-align: left;",
            thead {
                tr {
                    th { style: "padding: 10px", colspan: "2", "Used" }
                    th { style: "width:50%",
                        table {
                            tr {
                                td {
                                    style: "cursor:pointer",
                                    onclick: move |_| {
                                        component_state.write().order_by = OrderBy::Name;
                                    },
                                    "Name {name_up}"
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
                            component_state.write().order_by = OrderBy::Updated;
                        },
                        "Updated {updated_up}"
                    }
                    th {
                        div {
                            button {
                                class: "btn btn-sm btn-primary",
                                onclick: move |_| {
                                    let env_id = selected_env_id.clone();
                                    consume_context::<Signal<DialogState>>()
                                        .set(DialogState::EditSecret {
                                            env_id: env_id.clone(),
                                            name: "".to_string().into(),
                                            on_ok: EventHandler::new(move |result: EditSecretResult| {
                                                exec_save_secret(env_id.to_string(), result);
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

fn exec_save_secret(env_id: String, result: EditSecretResult) {
    spawn(async move {
        match crate::api::secrets::save_secret(env_id, result.name, result.value, result.level)
            .await
        {
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

fn exec_delete_secret(env_id: String, secret_id: String) {
    spawn(async move {
        match crate::api::secrets::delete_secret(env_id, secret_id).await {
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

fn get_last_edited(secrets: &Vec<SecretHttpModel>) -> String {
    let mut max = 0;

    let mut value = "".to_string();

    for secret in secrets {
        if secret.updated > 0 {
            if secret.updated > max {
                max = secret.updated;
                value = secret.name.clone();
            }
        }
    }

    value
}
pub struct SecretsListState {
    pub order_by: OrderBy,
}

impl SecretsListState {
    pub fn new() -> Self {
        Self {
            order_by: OrderBy::Name,
        }
    }

    pub fn sort<'a>(
        &self,
        secrets: impl Iterator<Item = &'a SecretHttpModel>,
    ) -> BTreeMap<String, &'a SecretHttpModel> {
        let mut result = BTreeMap::new();

        match self.order_by {
            OrderBy::Name => {
                for secret in secrets {
                    result.insert(secret.name.clone(), secret);
                }
            }

            OrderBy::Updated => {
                for secret in secrets {
                    result.insert(
                        crate::utils::unix_microseconds_to_string(secret.updated).into_string(),
                        secret,
                    );
                }
            }
        }

        result
    }
}
