use std::rc::Rc;

use dioxus::prelude::*;

use crate::{icons::*, states::MainState};

use super::*;

const NOT_SELECTED: &'static str = "---NOT SELECTED---";

#[component]
pub fn CopyToEnvDialog(from_env_id: Rc<String>, on_ok: EventHandler<String>) -> Element {
    let mut cs = use_signal(|| CopyToEnvState::new());
    let cs_ra = cs.read();

    let main_state = consume_context::<Signal<MainState>>();
    let main_state_ra = main_state.read();

    let mut options = Vec::new();

    options.push(rsx! {
        option { value: NOT_SELECTED, {NOT_SELECTED} }
    });

    for env in main_state_ra.envs.unwrap_as_loaded() {
        if env.as_str() == from_env_id.as_str() {
            continue;
        }

        options.push(rsx! {
            option { value: env.as_str(), {env.as_str()} }
        });
    }

    let content = rsx! {

        div { class: "form-floating mb-3",
            select {
                class: "form-control",
                oninput: move |cx| {
                    let value = cx.value();
                    let value = if value == NOT_SELECTED { String::new() } else { value };
                    cs.write().selected_env = value;
                },
                value: cs_ra.selected_env.as_str(),
            }
            label { "Secret name" }
        }
    };

    rsx! {

        DialogTemplate {
            header: "Select environment to copy",
            content,
            ok_button: rsx! {
                button {
                    class: "btn btn-primary",
                    disabled: cs_ra.save_button_is_disabled(),
                    onclick: move |_| {
                        let result = cs.read().selected_env.clone();
                        on_ok.call(result);
                        consume_context::<Signal<DialogState>>().set(DialogState::None);
                    },
                    OkButtonIcon {}
                    "Save"
                }
            },
        }
    }
}

pub struct CopyToEnvState {
    selected_env: String,
}

impl CopyToEnvState {
    pub fn new() -> Self {
        Self {
            selected_env: String::new(),
        }
    }

    pub fn save_button_is_disabled(&self) -> bool {
        self.selected_env.is_empty()
    }
}
