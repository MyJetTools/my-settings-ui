use std::rc::Rc;

use dioxus::prelude::*;

use dioxus_utils::*;

use crate::{dialogs::*, models::*};

use super::states::*;

#[component]
pub fn EditTemplate(
    env_id: Rc<String>,
    data: EditTemplateDialogData,
    on_ok: EventHandler<UpdateTemplateHttpModel>,
) -> Element {
    let mut cs = use_signal(move || EditTemplateState::new(data));

    let cs_ra = cs.read();

    if let Some(init_data) = cs_ra.init_from_other_template.as_ref() {
        match get_data(cs, init_data, &env_id) {
            Ok(_) => (),
            Err(err) => {
                return err;
            }
        }
    }

    let tabs_content = match cs_ra.tabs {
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
                                cs.write().tabs = EditTemplateTab::PeekSecret;
                            },
                            "Peek secret"
                        }
                    }
                }
                ChooseSecret {
                    env_id: env_id.clone(),
                    product_id: cs_ra.product_id.get_value(),
                    on_selected: move |selected: String| {
                        cs.write().add_secret_to_yaml(selected.as_str());
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
                                cs.write().tabs = EditTemplateTab::ChooseSecret;
                            },
                            "Choose secret"
                        }
                    }
                    li { class: "nav-item",
                        a { class: "nav-link  active", "Peek secret" }
                    }
                }
                PeekSecrets {
                    env_id: env_id.clone(),
                    product_id: cs_ra.product_id.get_value(),
                    yaml: cs_ra.yaml.get_value(),
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
                            disabled: !cs_ra.is_new_template(),
                            oninput: move |cx| {
                                cs.write().product_id.set_value(cx.value());
                            },
                            value: cs_ra.product_id.get_value().as_str(),
                        }

                        label { "ProductId" }
                    }

                    div { class: "form-floating mb-3",
                        input {
                            class: "form-control",
                            disabled: !cs_ra.is_new_template(),
                            oninput: move |cx| {
                                cs.write().template_id.set_value(cx.value());
                            },
                            value: cs_ra.template_id.as_str(),
                        }
                        label { "Name" }
                    }
                    div { class: "form-floating mb-3",
                        textarea {
                            class: "form-control",
                            style: "min-height:500px;font-family: monospace;",
                            oninput: move |cx| {
                                cs.write().yaml.set_value(cx.value());
                            },
                            value: cs_ra.yaml.as_str(),
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
                    disabled: cs_ra.save_button_disabled(),
                    onclick: move |_| {
                        let read_access = cs.read();
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
    mut cs: Signal<EditTemplateState>,
    init_data: &LoadDataFromTemplate,
    env_id: &Rc<String>,
) -> Result<(), Element> {
    match init_data.init_status.as_ref() {
        RenderState::None => {
            let env_id = env_id.to_string();
            let product_id = init_data.src_template.product_id.to_string();
            let template_id = init_data.src_template.template_id.to_string();
            spawn(async move {
                cs.write()
                    .init_from_other_template
                    .as_mut()
                    .unwrap()
                    .init_status
                    .set_loading();
                match crate::api::templates::get_template_content(
                    env_id.to_string(),
                    product_id,
                    template_id,
                )
                .await
                {
                    Ok(data) => {
                        let mut write_access = cs.write();
                        write_access.yaml.init(data);
                        write_access
                            .init_from_other_template
                            .as_mut()
                            .unwrap()
                            .init_status
                            .set_value(());
                    }
                    Err(err) => {
                        cs.write()
                            .init_from_other_template
                            .as_mut()
                            .unwrap()
                            .init_status
                            .set_error(err.to_string());
                    }
                }
            });
            return Err(crate::icons::loading_icon());
        }
        RenderState::Loading => {
            return Err(crate::icons::loading_icon());
        }
        RenderState::Loaded(_) => Ok(()),
        RenderState::Error(err) => {
            return Err(crate::icons::render_error(err));
        }
    }
}
