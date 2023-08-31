use std::rc::Rc;

use dioxus::prelude::*;

use crate::{
    states::{DialogState, MainState},
    views::icons::*,
};

#[derive(Props, PartialEq, Eq)]
pub struct EditSecretProps {
    pub secret: String,
}

pub struct LoadedValue {
    pub value: String,
    pub level: i32,
}

pub struct EditSecretState {
    pub init_name: Rc<String>,
    pub name: String,
    pub value: String,
    pub level: Option<i32>,
    pub loaded_value: Option<Rc<LoadedValue>>,
}

impl EditSecretState {
    pub fn new(secret: String) -> Self {
        Self {
            init_name: Rc::new(secret.clone()),
            name: secret.clone(),
            value: "".to_string(),
            level: None,
            loaded_value: None,
        }
    }

    pub fn get_secret_name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn get_secret_value(&self) -> &str {
        &self.value
    }

    pub fn get_secret_level(&self) -> String {
        match self.level {
            Some(level) => level.to_string(),
            None => "".to_string(),
        }
    }

    pub fn get_secret_level_value(&self) -> i32 {
        match self.level {
            Some(level) => level,
            None => 0,
        }
    }

    pub fn edit_mode(&self) -> bool {
        self.init_name.as_ref().len() > 0
    }

    pub fn value_is_loaded(&self) -> bool {
        self.loaded_value.is_some()
    }
    pub fn can_be_saved(&self) -> bool {
        if let Some(loaded_value) = self.loaded_value.as_ref() {
            if loaded_value.value == self.value && loaded_value.level == self.level.unwrap_or(0) {
                return false;
            }
        }

        self.name.len() > 0 && self.value.len() > 0 && self.level.is_some()
    }

    pub fn set_loaded_values(&self, value: String, level: i32) -> Self {
        Self {
            value: value.clone(),
            init_name: self.init_name.clone(),
            name: self.name.clone(),
            level: Some(level),
            loaded_value: Some(Rc::new(LoadedValue { value, level })),
        }
    }

    pub fn set_name(&self, name: String) -> Self {
        Self {
            value: self.value.clone(),
            init_name: self.init_name.clone(),
            name,
            level: self.level,
            loaded_value: self.loaded_value.clone(),
        }
    }

    pub fn set_value(&self, value: String) -> Self {
        Self {
            value,
            init_name: self.init_name.clone(),
            name: self.name.clone(),
            level: self.level,
            loaded_value: self.loaded_value.clone(),
        }
    }

    pub fn set_level(&self, level: Option<i32>) -> Self {
        Self {
            value: self.value.clone(),
            init_name: self.init_name.clone(),
            name: self.name.clone(),
            level,
            loaded_value: self.loaded_value.clone(),
        }
    }
}

pub fn edit_secret<'s>(cx: Scope<'s, EditSecretProps>) -> Element {
    let dialog_state = use_state(cx, || EditSecretState::new(cx.props.secret.clone()));

    let (edit_mode, value_is_loaded, save_button_is_disabled, name_is_read_only) = {
        let read_access = dialog_state.get();

        (
            read_access.edit_mode(),
            read_access.value_is_loaded(),
            !read_access.can_be_saved(),
            read_access.edit_mode(),
        )
    };

    if edit_mode && !value_is_loaded {
        load_secret(&cx, cx.props.secret.to_string(), &dialog_state);
    }

    render! {
        div { class: "modal-content",
            div { class: "form-floating mb-3",
                input {
                    class: "form-control",
                    readonly: name_is_read_only,
                    oninput: move |cx| {
                        dialog_state.modify(|itm| itm.set_name(cx.value.trim().to_string()));
                    },
                    value: "{dialog_state.get().get_secret_name()}"
                }
                label { "Secret name" }
            }

            div { class: "form-floating mb-3",
                input {
                    class: "form-control",
                    oninput: move |cx| {
                        dialog_state.modify(|itm| itm.set_value(cx.value.trim().to_string()));
                    },
                    value: "{dialog_state.get().get_secret_value()}"
                }
                label { "Secret value" }
            }

            div { class: "form-floating mb-3",
                input {
                    class: "form-control",
                    r#type: "number",
                    oninput: move |cx| {
                        dialog_state.modify(|itm| itm.set_level(cx.value.trim().parse::<i32>().ok()));
                    },
                    value: "{dialog_state.get().get_secret_level()}"
                }
                label { "Secret level" }
            }
        }
        div { class: "modal-footer",
            div { class: "btn-group",
                button {
                    class: "btn btn-primary",
                    disabled: save_button_is_disabled,
                    onclick: move |_| {
                        save_secret(cx, &dialog_state);
                    },
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

fn load_secret<'s>(
    cx: &'s Scope<'s, EditSecretProps>,
    secret_id: String,
    dialog_state: &UseState<EditSecretState>,
) {
    let dialog_state = dialog_state.to_owned();

    cx.spawn(async move {
        let response = crate::grpc_client::SecretsGrpcClient::get_secret(secret_id)
            .await
            .unwrap();
        dialog_state.modify(|itm| itm.set_loaded_values(response.value, response.level));
    });
}

fn save_secret<'s>(
    cx: &'s Scoped<'s, EditSecretProps>,
    edit_secret_state: &UseState<EditSecretState>,
) {
    let (name, value, level) = {
        let read_access = edit_secret_state.get();
        (
            read_access.get_secret_name().to_string(),
            read_access.get_secret_value().to_string(),
            read_access.get_secret_level_value(),
        )
    };

    let main_state = use_shared_state::<MainState>(cx).unwrap().to_owned();
    let dialog_state: UseSharedState<DialogState> =
        use_shared_state::<DialogState>(cx).unwrap().to_owned();

    cx.spawn(async move {
        crate::grpc_client::SecretsGrpcClient::save_secret(name.clone(), value, level)
            .await
            .unwrap();

        dialog_state.write().hide_dialog();
        main_state.write().set_secrets(None);
    })
}
