use std::rc::Rc;

use dioxus::prelude::*;

use dioxus_utils::DataState;

use crate::{dialogs::*, models::*, views::icons::*};

use super::states::*;

#[component]
pub fn EditTemplate(
    env_id: Rc<String>,
    data: EditTemplateDialogData,
    on_ok: EventHandler<UpdateTemplateHttpModel>,
) -> Element {
    let mut component_state = use_signal(move || EditTemplateState::new(env_id, data));

    let cs_read_access = component_state.read();

    if let Some(init_data) = cs_read_access.init_from_other_template.as_ref() {
        match get_data(component_state, &cs_read_access, init_data) {
            Ok(_) => (),
            Err(err) => {
                return err;
            }
        }
    }

    let tabs_content = match cs_read_access.tabs {
        EditTemplateTab::ChooseSecret => {
            rsx! {
                ul { class: "nav nav-tabs",
                    li { class: "nav-item",
                        a { class: "nav-link active", "Choose secret" }
                    }
                    li { class: "nav-item",
                        a {
                            class: "nav-link",
                            style: "cursor:pointer",
                            onclick: move |_| {
                                component_state.write().tabs = EditTemplateTab::PeekSecret;
                            },
                            "Peek secret"
                        }
                    }
                }
                ChooseSecret {
                    env_id: cs_read_access.env_id.clone(),
                    on_selected: move |selected: String| {
                        component_state.write().add_secret_to_yaml(selected.as_str());
                    },
                }
            }
        }
        EditTemplateTab::PeekSecret => {
            rsx! {
                ul { class: "nav nav-tabs",
                    li { class: "nav-item",
                        a {
                            class: "nav-link",
                            style: "cursor:pointer",
                            onclick: move |_| {
                                component_state.write().tabs = EditTemplateTab::ChooseSecret;
                            },
                            "Choose secret"
                        }
                    }
                    li { class: "nav-item",
                        a { class: "nav-link  active", "Peek secret" }
                    }
                }
                PeekSecrets {
                    env_id: cs_read_access.env_id.clone(),
                    yaml: cs_read_access.yaml.get_value(),
                }
            }
        }
    };

    let content = rsx! {
        table { style: "width:100%",
            tr {
                td { style: "width:60%",
                    div { class: "form-floating mb-3",
                        input {
                            class: "form-control",
                            disabled: !cs_read_access.is_new_template(),
                            oninput: move |cx| {
                                component_state.write().env.set_value(cx.value());
                            },
                            value: cs_read_access.env.as_str(),
                        }

                        label { "Env" }
                    }

                    div { class: "form-floating mb-3",
                        input {
                            class: "form-control",
                            disabled: !cs_read_access.is_new_template(),
                            oninput: move |cx| {
                                component_state.write().name.set_value(cx.value());
                            },
                            value: cs_read_access.name.as_str(),
                        }
                        label { "Name" }
                    }
                    div { class: "form-floating mb-3",
                        textarea {
                            class: "form-control",
                            style: "min-height:500px;font-family: monospace;",
                            oninput: move |cx| {
                                component_state.write().yaml.set_value(cx.value());
                            },
                            value: cs_read_access.yaml.as_str(),
                        }
                        label { "Yaml" }
                    }
                }
                td { style: "vertical-align:top", {tabs_content} }
            }
        }
    };

    rsx! {

        DialogTemplate {
            header: "Edit template",
            width: "95%",
            content,
            ok_button: rsx! {
                button {
                    class: "btn btn-primary",
                    disabled: cs_read_access.save_button_disabled(),
                    onclick: move |_| {
                        let read_access = component_state.read();
                        let result = read_access.unwrap_into_http_model();
                        on_ok.call(result);
                    },
                    "Save"
                }
            },
        }
    }
}

fn get_data(
    mut component_state: Signal<EditTemplateState>,
    cs_read_access: &EditTemplateState,
    init_data: &LoadDataFromTemplate,
) -> Result<(), Element> {
    match &init_data.init_status {
        DataState::None => {
            let env_id = cs_read_access.env_id.clone();
            let env = init_data.src_template.env.to_string();
            let name = init_data.src_template.name.to_string();
            spawn(async move {
                component_state
                    .write()
                    .init_from_other_template
                    .as_mut()
                    .unwrap()
                    .init_status = DataState::Loading;
                match crate::api::templates::get_template_content(env_id.to_string(), env, name)
                    .await
                {
                    Ok(data) => {
                        let mut write_access = component_state.write();
                        write_access.yaml.init(data);
                        write_access
                            .init_from_other_template
                            .as_mut()
                            .unwrap()
                            .init_status
                            .set_value(());
                    }
                    Err(err) => {
                        component_state
                            .write()
                            .init_from_other_template
                            .as_mut()
                            .unwrap()
                            .init_status
                            .set_error(err.to_string());
                    }
                }
            });
            return Err(rsx! {
                LoadingIcon {}
            });
        }
        DataState::Loading => {
            return Err(rsx! {
                LoadingIcon {}
            });
        }
        DataState::Loaded(_) => Ok(()),
        DataState::Error(err) => {
            return Err(rsx! {
                div { {err.as_str()} }
            })
        }
    }
}
