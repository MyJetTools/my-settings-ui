use std::{rc::Rc, collections::BTreeMap};

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use serde::*;

use crate::{
    states::{DialogState, DialogType, MainState},
    views::icons::*
};

pub enum OrderBy{
    Name,
    Updated,
}

pub fn secrets_list(cx: Scope) -> Element {
    let main_state = use_shared_state::<MainState>(cx).unwrap();

    let filter_secret = use_state(cx, || "".to_string());

    let value_to_filter = filter_secret.get().to_lowercase();


    let order_by = use_state(cx, || OrderBy::Name);


    match main_state.read().unwrap_as_secrets() {
        Some(secrets) => {
            let last_edited = get_last_edited(&secrets);

            let mut sorted = BTreeMap::new();

            let mut name_title = vec![rsx!{"Name"}];
            let mut updated_title = vec![rsx!{"Updated"}];

            match order_by.get(){
                OrderBy::Name => {
                    for secret in secrets{
                    sorted.insert(secret.name.clone(), secret);
                  
                };  
                name_title.push(rsx!{ table_up_icon {} });
            },
         
                OrderBy::Updated => {for secret in secrets{
                    sorted.insert(crate::utils::unix_microseconds_to_string(secret.updated) , secret);
                    
                }updated_title.push(rsx!{ table_up_icon {} });
            },
   
            }


            let secrets = sorted.values().
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

                let last_edited = if itm.name.as_str() == last_edited.as_str() {
                    Some(rsx!(
                        span { id: "last-edited-badge", class: "badge badge-success ", "Last edited" }
                        script { r#"scroll_to('last-edited-badge')"# }
                    ))
                }else{
                    None
                };

                let created = crate::utils::unix_microseconds_to_string(itm.created);
                let updated = crate::utils::unix_microseconds_to_string(itm.updated);
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
                        td { "{created}" }
                        td { "{updated}" }
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
                                    td {
                                        style: "cursor:pointer",
                                        onclick: move |_| {
                                            order_by.set(OrderBy::Name);
                                        },
                                        name_title.into_iter()
                                    }
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
                        th {
                            style: "cursor:pointer",
                            onclick: move |_| {
                                order_by.set(OrderBy::Updated);
                            },
                            updated_title.into_iter()
                        }
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

                    secrets.into_iter()
                }
            }
        }
        None => {

            let main_state= main_state.to_owned();
            cx.spawn(async move{
                let result = load_secrets().await.unwrap();
                main_state.write().set_secrets(Some(result));
            });
            
            render! { h1 { "loading" } }
        }
    }
}


fn get_last_edited(secrets: &Vec<SecretListItemApiModel>)->String{

    let mut max = 0;

    let mut value = "".to_string();

    for secret in secrets{

        if secret.updated>0{
            if secret.updated>max{
                max = secret.updated;
                value = secret.name.clone();
            }

        }
    }

    value

}

#[derive(Serialize, Deserialize, Debug)]
pub struct SecretListItemApiModel{
    pub name: String,
    pub level: i32, 
    pub created: i64,
    pub updated: i64,
    pub used_by_templates: i32,
    pub used_by_secrets: i32,
}


#[server]
pub async fn load_secrets() -> Result<Vec<SecretListItemApiModel>, ServerFnError> {
    let response = crate::grpc_client::SecretsGrpcClient::get_all_secrets()
    .await
    .unwrap();

    use rust_extensions::date_time::DateTimeAsMicroseconds;

    let result = response.into_iter().map(|itm|SecretListItemApiModel{
         name: itm.name, 
         level: itm.level, 
         created: DateTimeAsMicroseconds::from_str(itm.created.as_str()).unwrap().unix_microseconds,
         updated: DateTimeAsMicroseconds::from_str(itm.updated.as_str()).unwrap().unix_microseconds, 
         used_by_templates: itm.used_by_templates,
         used_by_secrets: itm.used_by_secrets }).collect();

    Ok(result)

}