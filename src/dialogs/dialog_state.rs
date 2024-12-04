use std::rc::Rc;

use dioxus::prelude::EventHandler;

use crate::dialogs::*;

#[derive(Debug, Clone)]
pub enum DialogState {
    None,
    Confirmation {
        content: String,
        on_ok: EventHandler<()>,
    },
    ShowSecret(Rc<String>),
    EditSecret {
        name: Rc<String>,
        on_ok: EventHandler<EditSecretResult>,
    },

    EditTemplate {
        env: Rc<String>,
        name: Rc<String>,
        init_from_other_template: Option<(Rc<String>, Rc<String>)>,
        on_ok: EventHandler<SaveTemplateResult>,
    },
    ShowPopulatedYaml {
        env: Rc<String>,
        name: Rc<String>,
    },
    SecretUsage(Rc<String>),
    SecretUsageBySecret(Rc<String>),
}
