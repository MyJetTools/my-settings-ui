use std::rc::Rc;

use dioxus::prelude::*;

use crate::{
    states::{DialogState, DialogType, MainState, LastEdited},
    views::icons::*,
};

pub fn secrets_list(cx: Scope) -> Element {
    let main_state = use_shared_state::<MainState>(cx).unwrap();

    let filter_secret = use_state(cx, || "".to_string());

    let value_to_filter = filter_secret.get().to_lowercase();

    let last_edited = use_shared_state::<LastEdited>(cx).unwrap().read().get_secret();

    match main_state.read().unwrap_as_secrets() {
        Some(templates) => {
            let templates = templates.iter().
            filter(|itm|{
                if value_to_filter.len() == 0 {
                    return true;
                }

                itm.name.to_lowercase().contains(&value_to_filter)

            }).map(|itm| {
                let secret = Rc::new(itm.name.to_string());
                let secret2 = secret.clone();
                let secret3 = secret.clone();
                let edit_button_secret = secret.clone();
                let delete_secret_button = secret.clone();

                let mut class_template =  "badge badge-success empty-links";
                let mut class_secret =  class_template;
                if itm.used_by_templates == 0 && itm.used_by_secrets == 0 {
                    class_template = "badge badge-danger have-no-links";
                    class_secret = class_template;
                } else {
                    if itm.used_by_templates > 0 {
                        class_template =  "badge badge-success have-links";
                    }

                    if itm.used_by_secrets > 0 {
                        class_secret =  "badge badge-success have-links";
                    }
                   
                };

                let secret_amount = itm.used_by_secrets;
                let templates_amount = itm.used_by_templates;

                let last_edited = if &itm.name == last_edited.as_ref() {
                    Some(rsx!(
                        span { id: "last-edited-badge", class: "badge badge-success ", "Last edited" }
                        script { r#"scroll_to('last-edited-badge')"# }
                    ))
                }else{
                    None
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
                                        let dialog_state = use_shared_state::<DialogState>(cx).unwrap();
                                        dialog_state
                                            .write()
                                            .show_dialog(
                                                format!("Show secret '{}' usage", secret2),
                                                DialogType::SecretUsage(secret2.to_string()),
                                            );
                                    },
                                    "{itm.used_by_templates}"
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
                                        let dialog_state = use_shared_state::<DialogState>(cx).unwrap();
                                        dialog_state
                                            .write()
                                            .show_dialog(
                                                format!("Show secret '{}' usage", secret3),
                                                DialogType::SecretUsageBySecret(secret3.to_string()),
                                            );
                                    },
                                    "{itm.used_by_secrets}"
                                }
                            }
                        }
                        td { style: "padding: 10px", "{itm.name}", last_edited }
                        td { "{itm.level}" }
                        td { "{itm.created}" }
                        td { "{itm.updated}" }
                        td {
                            div { class: "btn-group",
                                button {
                                    class: "btn btn-sm btn-success",
                                    onclick: move |_| {
                                        let dialog_state = use_shared_state::<DialogState>(cx).unwrap();
                                        dialog_state
                                            .write()
                                            .show_dialog(
                                                format!("Show [{}] secret value", secret),
                                                DialogType::ShowSecret(secret.to_string()),
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
                                                format!("Edit secret").to_string(),
                                                DialogType::EditSecret(edit_button_secret.to_string()),
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
                                                format!("Delete secret {}", delete_secret_button.as_str()).to_string(),
                                                DialogType::DeleteSecret(delete_secret_button.to_string()),
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
                        th { style: "padding: 10px", colspan: "2", "Used" }
                        th { style: "width:100%",
                            table {
                                tr {
                                    td { "Name" }
                                    td { style: "width:100%",
                                        div { class: "input-group",
                                            span { class: "input-group-text", search_icon {} }
                                            input {
                                                class: "form-control form-control-sm",
                                                oninput: move |cx| {
                                                    filter_secret.set(cx.value.to_string());
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        th { "Level" }
                        th { "Created" }
                        th { "Updated" }
                        th {
                            div {
                                button {
                                    class: "btn btn-sm btn-primary",
                                    onclick: move |_| {
                                        let dialog_state = use_shared_state::<DialogState>(cx).unwrap();
                                        dialog_state.write().show_dialog("Add secret".to_string(), DialogType::AddSecret);
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
       // let response = crate::api_client::get_list_of_secrets().await.unwrap();

       let response = crate::grpc_client::SecretsGrpcClient::get_all_secrets()
            .await
            .unwrap();

        main_state.write().set_secrets(Some(response));
    });
}
