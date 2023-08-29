use dioxus::prelude::*;

use crate::{
    states::{DialogState, DialogType, MainState},
    views::icons::*,
};

pub fn secrets_list(cx: Scope) -> Element {
    let main_state = use_shared_state::<MainState>(cx).unwrap();

    let filter_secret = use_state(cx, || "".to_string());

    let value_to_filter = filter_secret.get().to_lowercase();

    match main_state.read().unwrap_as_secrets() {
        Some(templates) => {
            let templates = templates.iter().
            filter(|itm|{
                if value_to_filter.len() == 0 {
                    return true;
                }

                itm.name.to_lowercase().contains(&value_to_filter)

            }).map(|itm| {
                let secret = itm.name.to_string();
                let secret2 = itm.name.to_string();
                let secret3 = itm.name.to_string();

                let mut class_template =  "badge badge-success empty-links";
                let mut class_secret =  class_template;
                if itm.templates_amount == 0 && itm.secrets_amount == 0 {
                    class_template = "badge badge-danger have-no-links";
                    class_secret = class_template;
                } else {
                    if itm.templates_amount > 0 {
                        class_template =  "badge badge-success have-links";
                    }

                    if itm.secrets_amount > 0 {
                        class_secret =  "badge badge-success have-links";
                    }
                   
                };

                let secret_amount = itm.secrets_amount;
                let templates_amount = itm.templates_amount;

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
                                                DialogType::SecretUsage(secret2.clone()),
                                            );
                                    },
                                    "{itm.templates_amount}"
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
                                                DialogType::SecretUsageBySecret(secret3.clone()),
                                            );
                                    },
                                    "{itm.secrets_amount}"
                                }
                            }
                        }
                        td { style: "padding: 10px", "{itm.name}" }
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
                                                DialogType::ShowSecret(secret.clone()),
                                            );
                                    },
                                    view_template_icon {}
                                }
                                button { class: "btn btn-sm btn-primary", edit_icon {} }
                                button { class: "btn btn-sm btn-danger", delete_icon {} }
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
        let response = crate::api_client::get_list_of_secrets().await.unwrap();

        main_state.write().set_secrets(Some(response));
    });
}
