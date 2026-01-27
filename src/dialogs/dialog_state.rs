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
        product_id: Option<Rc<String>>,
        secret_id: Rc<String>,
    },
    EditSecret {
        env_id: Rc<String>,
        product_id: Option<Rc<String>>,
        secret_id: Rc<String>,
        on_ok: EventHandler<EditSecretResult>,
    },

    EditTemplate {
        env_id: Rc<String>,
        data: EditTemplateDialogData,
        on_ok: EventHandler<UpdateTemplateHttpModel>,
    },
    ShowPopulatedYaml {
        env_id: Rc<String>,
        product_id: Rc<String>,
        template_id: Rc<String>,
    },
    SecretUsage {
        env_id: Rc<String>,
        product_id: Option<Rc<String>>,
        secret_id: Rc<String>,
    },
    SecretUsageBySecret {
        env_id: Rc<String>,
        secret_id: Rc<String>,
    },
    SnapshotToExport(Rc<String>),

    SnapshotToImport(EventHandler<String>),

    CopyToEnvConfirmation {
        from_env_id: Rc<String>,
        on_ok: EventHandler<String>,
    },
}
