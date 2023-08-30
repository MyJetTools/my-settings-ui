use std::rc::Rc;

use dioxus::prelude::*;

use crate::{
    states::{DialogState, LastEdited, MainState},
    views::icons::*,
};

#[derive(Props, PartialEq, Eq)]
pub struct EditTemplateProps {
    pub env: String,
    pub name: String,
}
pub fn edit_template<'s>(cx: Scope<'s, EditTemplateProps>) -> Element {
    let edit_state = use_state(cx, || {
        EditTemplateState::new(cx.props.env.to_string(), cx.props.name.to_string())
    });

    let (edit_mode, yaml_loaded, env_name_read_only, save_button_disabled) = {
        let edit_state = edit_state.get();

        (
            edit_state.is_edit_mode(),
            edit_state.is_loaded(),
            edit_state.is_edit_mode(),
            !edit_state.save_button_enabled(),
        )
    };

    if edit_mode && !yaml_loaded {
        load_template(&cx, edit_state);
    }

    render! {
        div { class: "modal-content",
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

pub fn load_template<'s>(
    cx: &'s Scope<'s, EditTemplateProps>,
    state: &UseState<EditTemplateState>,
) {
    let env = state.get_env().to_string();
    let name = state.get_name().to_string();

    let state = state.to_owned();

    cx.spawn(async move {
        let yaml = crate::grpc_client::TemplatesGrpcClient::get_template(env, name)
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
    yaml: String,
    loaded: Option<Rc<String>>,
}

impl EditTemplateState {
    pub fn new(env: String, name: String) -> Self {
        Self {
            edit_mode: env.len() > 0,
            env: env.to_string(),
            name: name.to_string(),

            yaml: "".to_string(),
            loaded: None,
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
        }
    }

    pub fn set_name(&self, name: String) -> Self {
        Self {
            env: self.env.to_string(),
            name,
            edit_mode: self.edit_mode,
            yaml: self.yaml.to_string(),
            loaded: self.loaded.clone(),
        }
    }

    pub fn set_yaml(&self, yaml: String) -> Self {
        Self {
            env: self.env.to_string(),
            name: self.name.to_string(),
            edit_mode: self.edit_mode,
            yaml,
            loaded: self.loaded.clone(),
        }
    }

    pub fn loaded_yaml(&self, yaml: String) -> Self {
        Self {
            env: self.env.to_string(),
            name: self.name.to_string(),
            edit_mode: self.edit_mode,
            yaml: yaml.clone(),
            loaded: Some(Rc::new(yaml)),
        }
    }
}