use std::rc::Rc;

use dioxus::prelude::*;

use dioxus_utils::DataState;
use serde::*;

use crate::{dialogs::*, views::icons::*};

#[component]
pub fn EditTemplate(
    env_id: Rc<String>,
    env: String,
    name: Rc<String>,
    init_from_other_template: Option<(Rc<String>, Rc<String>)>,
    on_ok: EventHandler<SaveTemplateResult>,
) -> Element {
    let mut component_state =
        use_signal(|| EditTemplateState::new(env, name.to_string(), init_from_other_template));

    let component_state_read_access = component_state.read();

    if let Some(init_data) = component_state_read_access
        .init_from_other_template
        .as_ref()
    {
        match &component_state_read_access.init_data {
            DataState::None => {
                let env_id = env_id.clone();
                let env = init_data.0.to_string();
                let name = init_data.1.to_string();
                spawn(async move {
                    component_state.write().init_data = DataState::Loading;
                    match load_template(env_id.to_string(), env, name).await {
                        Ok(data) => {
                            component_state.write().init_from_other_template(data.yaml);
                            component_state.write().init_data = DataState::Loaded(());
                        }
                        Err(err) => {
                            component_state.write().init_data = DataState::Error(err.to_string());
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
                };
            }
            DataState::Loaded(_) => {}
            DataState::Error(err) => {
                return rsx! {
                    div { {err.as_str()} }
                }
            }
        }
    }

    let tabs_content = match component_state_read_access.tabs {
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
                    env_id: env_id.clone(),
                    on_selected: move |selected: String| {
                        component_state.write().add_secret_to_yaml(selected.as_str());
                    }
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
                PeekSecrets { env_id: env_id.clone(), yaml: component_state_read_access.yaml.as_str() }
            }
        }
    };

    match component_state_read_access.yaml_from_db.as_ref() {
        DataState::None => {
            let env_id = env_id.clone();
            let env = component_state_read_access.env.to_string();
            let name = component_state_read_access.name.to_string();
            spawn(async move {
                component_state.write().yaml_from_db = DataState::Loading;
                match load_template(env_id.to_string(), env, name).await {
                    Ok(data) => {
                        component_state.write().init(data.yaml);
                    }
                    Err(err) => {
                        component_state.write().yaml_from_db = DataState::Error(err.to_string());
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
        DataState::Loaded(_) => {}
        DataState::Error(err) => {
            return rsx! {
                div { {err.as_str()} }
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
                            disabled: !component_state_read_access.is_new_template(),
                            oninput: move |cx| {
                                component_state.write().env = cx.value();
                            },
                            value: component_state_read_access.env.as_str()
                        }

                        label { "Env" }
                    }

                    div { class: "form-floating mb-3",
                        input {
                            class: "form-control",
                            disabled: !component_state_read_access.is_new_template(),
                            oninput: move |cx| {
                                component_state.write().name = cx.value();
                            },
                            value: component_state_read_access.name.as_str()
                        }
                        label { "Name" }
                    }
                    div { class: "form-floating mb-3",
                        textarea {
                            class: "form-control",
                            style: "min-height:500px;font-family: monospace;",
                            oninput: move |cx| {
                                component_state.write().yaml = cx.value();
                            },
                            value: component_state_read_access.yaml.as_str()
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
                    disabled: component_state_read_access.save_button_disabled(),
                    onclick: move |_| {
                        let result = component_state.read().get_result();
                        on_ok.call(result);
                    },
                    OkButtonIcon {}
                    "Save"
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoadedTemplate {
    pub yaml: String,
}

#[server]
pub async fn load_template(
    env_id: String,
    env: String,
    name: String,
) -> Result<LoadedTemplate, ServerFnError> {
    use crate::server::templates_grpc::*;
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    let response = ctx
        .templates_grpc
        .get(GetTemplateRequest { env, name })
        .await
        .unwrap();
    Ok(LoadedTemplate {
        yaml: response.yaml,
    })
}

pub struct SaveTemplateResult {
    pub env: String,
    pub name: String,
    pub yaml: String,
}

pub struct EditTemplateState {
    tabs: EditTemplateTab,
    env: String,
    name: String,
    new_template: bool,
    yaml_from_db: DataState<String>,
    yaml: String,
    init_from_other_template: Option<(Rc<String>, Rc<String>)>,
    init_data: DataState<()>,
}

impl EditTemplateState {
    pub fn new(
        env: String,
        name: String,
        init_from_other_template: Option<(Rc<String>, Rc<String>)>,
    ) -> Self {
        let new_template = env.len() == 0;

        Self {
            new_template,
            env,
            name,
            yaml: "".to_string(),
            tabs: EditTemplateTab::ChooseSecret,
            init_from_other_template,
            init_data: DataState::None,
            yaml_from_db: if new_template {
                DataState::Loaded("Empty".to_string())
            } else {
                DataState::None
            },
        }
    }

    pub fn init(&mut self, yaml: String) {
        self.yaml = yaml.to_string();
        self.yaml_from_db = DataState::Loaded(yaml);
    }

    pub fn init_from_other_template(&mut self, yaml: String) {
        self.yaml = yaml;
        self.yaml_from_db = DataState::Loaded("".to_string());
    }

    pub fn save_button_disabled(&self) -> bool {
        let yaml_from_db = match &self.yaml_from_db {
            DataState::Loaded(value) => value,
            _ => return true,
        };

        return self.yaml.as_str() == yaml_from_db
            || self.name.len() == 0
            || self.yaml.len() == 0
            || self.env.len() == 0;
    }

    pub fn is_new_template(&self) -> bool {
        self.new_template
    }

    pub fn add_secret_to_yaml(&mut self, value: &str) {
        self.yaml.push_str("${");
        self.yaml.push_str(value);
        self.yaml.push('}');
    }

    pub fn get_result(&self) -> SaveTemplateResult {
        SaveTemplateResult {
            env: self.env.to_string(),
            name: self.name.to_string(),
            yaml: self.yaml.to_string(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum EditTemplateTab {
    ChooseSecret,
    PeekSecret,
}
