use std::rc::Rc;

use dioxus::prelude::EventHandler;

use crate::{dialogs::*, models::UpdateTemplateHttpModel};

use super::states::EditTemplateDialogData;

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
        data: EditTemplateDialogData,
        on_ok: EventHandler<UpdateTemplateHttpModel>,
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
    SnapshotToExport(Rc<String>),

    SnapshotToImport(EventHandler<String>),

    CopyToEnvConfirmation {
        from_env_id: Rc<String>,
        on_ok: EventHandler<String>,
    },
}
