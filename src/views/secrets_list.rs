use std::{rc::Rc, collections::BTreeMap};

use dioxus::prelude::*;

use serde::*;

use crate::{
    dialogs::*, states::*, views::icons::*
};

#[derive(Debug, Clone, Copy)]
pub enum OrderBy{
    Name,
    Updated,
}

#[component]
pub fn SecretsList() -> Element {
    let mut main_state = consume_context::<Signal<MainState>>();

    let main_state_read_access = main_state.read();

    let mut filter_secret = consume_context::<Signal<FilterSecret>>();

    let filter_secret_read_access = filter_secret.read();
    let value_to_filter = filter_secret_read_access.as_str();


    let mut order_by_state = use_signal( || OrderBy::Name);

    let order_by_value = {
        let order_by_read_access = order_by_state.read();
        order_by_read_access.clone()
    };
    


    match main_state_read_access.unwrap_as_secrets() {
        Some(secrets) => {
            let last_edited = get_last_edited(&secrets);

            let mut sorted = BTreeMap::new();

            let mut name_title = vec![rsx!{"Name"}];
            let mut updated_title = vec![rsx!{"Updated"}];

            match order_by_value{
                OrderBy::Name => {
                    for secret in secrets{
                    sorted.insert(secret.name.clone(), secret);
                  
                };  
                name_title.push(rsx!{
                    TableUpIcon {}
                });
            },
         
                OrderBy::Updated => {for secret in secrets{
                    sorted.insert(crate::utils::unix_microseconds_to_string(secret.updated).into_string() , secret);
                    
                }updated_title.push(rsx!{
                    TableUpIcon {}
                });
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
                let secret_usage_name = secret.clone();
                let secret3 = secret.clone();
                let edit_button_secret_name = secret.clone();
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
                        span {
                            id: "last-edited-badge",
                            class: "badge badge-success not-selectable",
                            "Last edited"
                        }
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
                                        let secret_name = secret_usage_name.clone();
                                        consume_context::<Signal<DialogState>>()
                                            .set(DialogState::SecretUsage(secret_name))
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
                                        let name = secret3.clone();
                                        consume_context::<Signal<DialogState>>()
                                            .set(DialogState::SecretUsageBySecret(name));
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
                                button {
                                    class: "btn btn-sm btn-success",
                                    onclick: move |_| {
                                        let name = secret.clone();
                                        consume_context::<Signal<DialogState>>().set(DialogState::ShowSecret(name));
                                    },
                                    ViewTemplateIcon {}
                                }
                                button {
                                    class: "btn btn-sm btn-primary",
                                    onclick: move |_| {
                                        let name = edit_button_secret_name.clone();
                                        consume_context::<Signal<DialogState>>()
                                            .set(DialogState::EditSecret {
                                                name,
                                                on_ok: EventHandler::new(move |result: EditSecretResult| {
                                                    exec_save_secret(main_state, result);
                                                }),
                                            })
                                    },
                                    EditIcon {}
                                }
                                button {
                                    class: "btn btn-sm btn-danger",
                                    onclick: move |_| {
                                        consume_context::<Signal<DialogState>>()
                                            .set(DialogState::Confirmation {
                                                content: format!("Delete secret {}", delete_secret_button.as_str()),
                                                on_ok: EventHandler::new(move |_| {
                                                    spawn(async move {});
                                                }),
                                            });
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
                            th { style: "padding: 10px", colspan: "2", "Used" }
                            th { style: "width:50%",
                                table {
                                    tr {
                                        td {
                                            style: "cursor:pointer",
                                            onclick: move |_| {
                                                order_by_state.set(OrderBy::Name);
                                            },
                                            {name_title.into_iter()}
                                        }
                                        td { style: "width:90%",
                                            div { class: "input-group",
                                                span { class: "input-group-text", SearchIcon {} }
                                                input {
                                                    class: "form-control form-control-sm",
                                                    value: "{value_to_filter}",
                                                    oninput: move |cx| {
                                                        let mut write = filter_secret.write();
                                                        write.set_value(cx.value().as_str());
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
                                    order_by_state.set(OrderBy::Updated);
                                },
                                {updated_title.into_iter()}
                            }
                            th {
                                div {
                                    button {
                                        class: "btn btn-sm btn-primary",
                                        onclick: move |_| {
                                            consume_context::<Signal<DialogState>>()
                                                .set(DialogState::EditSecret {
                                                    name: "".to_string().into(),
                                                    on_ok: EventHandler::new(move |result: EditSecretResult| {
                                                        exec_save_secret(main_state, result);
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
        None => {
            
            spawn(async move{
                let result = load_secrets().await.unwrap();
                main_state.write().set_secrets(Some(result));
            });
            
            rsx! {
                LoadingIcon {}
            }
        }
    }
}

fn exec_save_secret(mut main_state : Signal<MainState>, result: EditSecretResult){
    spawn(async move { match save_secret(result.name, result.value, result.level).await{
        Ok(_) => {
            let mut write = main_state.write();
            write.set_secrets(None);
        },
        Err(_) => todo!(),
    } });
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
    let response = crate::server::grpc_client::SecretsGrpcClient::get_all_secrets()
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

#[server]
pub async fn save_secret(name: String, value: String, level: i32) -> Result<(), ServerFnError> {
    crate::server::grpc_client::SecretsGrpcClient::save_secret(name, value, level)
        .await
        .unwrap();

    Ok(())
}