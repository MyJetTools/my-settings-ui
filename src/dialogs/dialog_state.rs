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
    ShowSecret {
        env_id: Rc<String>,
        secret: Rc<String>,
    },
    EditSecret {
        env_id: Rc<String>,
        name: Rc<String>,
        on_ok: EventHandler<EditSecretResult>,
    },

    EditTemplate {
        env_id: Rc<String>,
        env: Rc<String>,
        name: Rc<String>,
        init_from_other_template: Option<(Rc<String>, Rc<String>)>,
        on_ok: EventHandler<SaveTemplateResult>,
    },
    ShowPopulatedYaml {
        env_id: Rc<String>,
        env: Rc<String>,
        name: Rc<String>,
    },
    SecretUsage {
        env_id: Rc<String>,
        secret: Rc<String>,
    },
    SecretUsageBySecret {
        env_id: Rc<String>,
        secret: Rc<String>,
    },
}
