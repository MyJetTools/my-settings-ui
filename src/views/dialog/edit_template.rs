use std::{collections::HashMap, rc::Rc, time::Duration};

use dioxus::prelude::*;

use crate::{
    secrets_grpc::SecretModel,
    states::{DialogState, LastEdited, MainState},
    views::icons::*,
};

#[derive(Props, PartialEq, Eq)]
pub struct EditTemplateProps {
    pub env: String,
    pub name: String,
    pub copy_from_template: bool,
}
pub fn edit_template<'s>(cx: Scope<'s, EditTemplateProps>) -> Element {
    let edit_state = use_state(cx, || {
        EditTemplateState::new(
            cx.props.env.to_string(),
            cx.props.name.to_string(),
            cx.props.copy_from_template,
        )
    });

    let secrets_state: &UseState<HashMap<String, Option<SecretModel>>> =
        use_state(cx, || HashMap::new());

    let (edit_mode, copy_from_model, yaml_loaded, env_name_read_only, save_button_disabled) = {
        let edit_state = edit_state.get();

        (
            edit_state.is_edit_mode(),
            edit_state.is_copy_from_model(),
            edit_state.is_loaded(),
            edit_state.is_edit_mode(),
            !edit_state.save_button_enabled(),
        )
    };

    if edit_mode && !yaml_loaded {
        load_template(&cx, &cx.props.env, &cx.props.name, edit_state);
    }

    if let Some(copy_from_model) = copy_from_model {
        load_template(&cx, &copy_from_model.0, &copy_from_model.1, edit_state);
    }

    let mut secrets = Vec::new();

    let yaml = edit_state.get_yaml();

    let secrets = if yaml.len() > 10 {
        for secret_name in settings_utils::placeholders::get_secret_names(edit_state.get_yaml()) {
            let secret_name_to_load = secret_name.to_string();
            let (secret_value, secret_level) = match secrets_state.get().get(secret_name) {
                Some(value) => match value {
                    Some(value) => (
                        rsx! { div { style:"font-size:12px; width:300px; height:32px; overflows-y:auto", "{value.value}" } },
                        rsx! {div{ style:"font-size:12px", "{value.level}"}},
                    ),
                    None => (
                        rsx! { div { style: "color:red", "Value not found" } },
                        rsx! {div{}},
                    ),
                },
                None => (
                    rsx! { div{class:"btn-group", button { class:"btn btn-primary btn-sm", onclick: move|_|{
                        load_secret(cx, &secret_name_to_load, secrets_state);
                    }, "Load" } }},
                    rsx! {div{}},
                ),
            };

            secrets.push(rsx! {
                tr {
                    td { style: "font-size:12px", "{secret_name}:" }
                    td { width: "100%", secret_value }
                    td { width: "30px", secret_level }
                }
            });
        }
        rsx! {
            table { class: "table table-striped",
                tr {
                    th { "secret" }
                    th { "value" }
                    th { "level" }
                }
                secrets.into_iter()
            }
        }
        .into()
    } else {
        None
    };

    render! {
        div { class: "modal-content",
            table { style: "width:100%",
                tr {
                    td { style: "width:60%",
                        div { class: "form-floating mb-3",
                            input {
                                class: "form-control",
                                readonly: env_name_read_only,
                                oninput: move |cx| {
                                    edit_state.modify(|itm| itm.set_env(cx.value.to_string()));
                                },
                                value: "{edit_state.get_env()}"
                            }

                            label { "Env" }
                        }

                        div { class: "form-floating mb-3",
                            input {
                                class: "form-control",
                                readonly: env_name_read_only,
                                oninput: move |cx| {
                                    edit_state.modify(|itm| itm.set_name(cx.value.to_string()));
                                },
                                value: "{edit_state.get_name()}"
                            }
                            label { "Name" }
                        }
                        div { class: "form-floating mb-3",
                            textarea {
                                class: "form-control",
                                style: "min-height:500px;font-family: monospace;",
                                oninput: move |cx| {
                                    edit_state.modify(|itm| itm.set_yaml(cx.value.to_string()));
                                },
                                value: "{edit_state.get_yaml()}"
                            }
                            label { "Yaml" }
                        }
                    }
                    td { style: "vertical-align:top", secrets }
                }
            }
        }
        div { class: "modal-footer",
            div { class: "btn-group",
                button {
                    class: "btn btn-primary",
                    disabled: save_button_disabled,
                    onclick: move |_| { save_template(&cx, edit_state) },
                    ok_button_icon {}
                    "Save"
                }
                button {
                    class: "btn btn-outline-dark",
                    onclick: move |_| {
                        use_shared_state::<DialogState>(cx).unwrap().write().hide_dialog();
                    },
                    cancel_button_icon {}
                    "Cancel"
                }
            }
        }
    }
}

pub fn load_secret<'s>(
    cx: &'s Scoped<'s, EditTemplateProps>,
    secret_name: &str,
    secrets: &UseState<HashMap<String, Option<SecretModel>>>,
) {
    let secret_name = secret_name.to_string();
    let secrets = secrets.to_owned();

    cx.spawn(async move {
        let secret_model = crate::grpc_client::SecretsGrpcClient::get_secret(secret_name.clone())
            .await
            .unwrap();

        if secret_model.name.len() > 0 {
            secrets.modify(|itm| {
                let mut secrets = itm.clone();
                secrets.insert(secret_name, Some(secret_model));
                secrets
            });
        } else {
            secrets.modify(|itm| {
                let mut secrets = itm.clone();
                secrets.insert(secret_name, None);
                secrets
            });
        }
    });
}

pub fn load_template<'s>(
    cx: &'s Scope<'s, EditTemplateProps>,
    env: &str,
    name: &str,
    state: &UseState<EditTemplateState>,
) {
    let env = env.to_string();
    let name = name.to_string();

    let state = state.to_owned();

    cx.spawn(async move {
        let yaml = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(300)).await;
            crate::grpc_client::TemplatesGrpcClient::get_template(env, name)
                .await
                .unwrap()
        })
        .await
        .unwrap();

        state.modify(|itm| itm.loaded_yaml(yaml));
    });
}

pub fn save_template<'s>(
    cx: &'s Scope<'s, EditTemplateProps>,
    state: &UseState<EditTemplateState>,
) {
    let env = state.get_env().to_string();
    let name = state.get_name().to_string();
    let yaml = state.get_yaml().to_string();

    let main_state = use_shared_state::<MainState>(cx).unwrap().to_owned();
    let dialog_state: UseSharedState<DialogState> =
        use_shared_state::<DialogState>(cx).unwrap().to_owned();

    let last_edited: UseSharedState<LastEdited> = use_shared_state(cx).unwrap().to_owned();

    cx.spawn(async move {
        crate::grpc_client::TemplatesGrpcClient::save_template(env.clone(), name.clone(), yaml)
            .await
            .unwrap();

        last_edited.write().set_last_template_edited(env, name);
        dialog_state.write().hide_dialog();
        main_state.write().set_templates(None);
    });
}

pub struct EditTemplateState {
    env: String,
    name: String,
    edit_mode: bool,
    copy_from: Option<(String, String)>,
    yaml: String,
    loaded: Option<Rc<String>>,
}

impl EditTemplateState {
    pub fn new(env: String, name: String, copy_from: bool) -> Self {
        let edit_mode = if copy_from { false } else { env.len() > 0 };

        if copy_from {
            Self {
                edit_mode,
                env: env.to_string(),
                name: "".to_string(),

                yaml: "".to_string(),
                loaded: None,
                copy_from: Some((env, name)),
            }
        } else {
            Self {
                edit_mode,
                env,
                name,
                yaml: "".to_string(),
                loaded: None,
                copy_from: None,
            }
        }
    }

    pub fn save_button_enabled(&self) -> bool {
        if self.is_edit_mode() {
            match &self.loaded {
                Some(loaded_yaml) => return self.yaml.as_str() != loaded_yaml.as_str(),
                None => return false,
            }
        }

        self.name.len() > 0 && self.yaml.len() > 0 && self.env.len() > 0
    }

    pub fn is_edit_mode(&self) -> bool {
        self.edit_mode
    }

    pub fn is_copy_from_model(&self) -> &Option<(String, String)> {
        &self.copy_from
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded.is_some()
    }

    pub fn get_env(&self) -> &str {
        self.env.as_str()
    }

    pub fn get_yaml(&self) -> &str {
        self.yaml.as_str()
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn set_env(&self, env: String) -> Self {
        Self {
            env,
            name: self.name.to_string(),
            edit_mode: self.edit_mode,
            yaml: self.yaml.to_string(),
            loaded: self.loaded.clone(),
            copy_from: self.copy_from.clone(),
        }
    }

    pub fn set_name(&self, name: String) -> Self {
        Self {
            env: self.env.to_string(),
            name,
            edit_mode: self.edit_mode,
            yaml: self.yaml.to_string(),
            loaded: self.loaded.clone(),
            copy_from: self.copy_from.clone(),
        }
    }

    pub fn set_yaml(&self, yaml: String) -> Self {
        Self {
            env: self.env.to_string(),
            name: self.name.to_string(),
            edit_mode: self.edit_mode,
            yaml,
            loaded: self.loaded.clone(),
            copy_from: self.copy_from.clone(),
        }
    }

    pub fn loaded_yaml(&self, yaml: String) -> Self {
        Self {
            env: self.env.to_string(),
            name: self.name.to_string(),
            edit_mode: self.edit_mode,
            yaml: yaml.clone(),
            loaded: Some(Rc::new(yaml)),
            copy_from: self.copy_from.clone(),
        }
    }
}
