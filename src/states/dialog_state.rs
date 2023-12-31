use std::rc::Rc;

use crate::views::NginxConfigHttpModel;

pub enum DialogType {
    ShowSecret(String),
    AddSecret,
    EditSecret(String),
    DeleteSecret(String),

    AddTemplate,
    AddTemplateFromOtherTemplate {
        env: String,
        name: String,
    },
    EditTemplate {
        env: String,
        name: String,
    },
    DeleteTemplate {
        env: String,
        name: String,
    },
    ShowPopulatedYaml {
        env: String,
        name: String,
    },
    SecretUsage(String),
    SecretUsageBySecret(String),

    AddDomainProduct,

    EditDomainProduct {
        name: String,
        cloud_flare_proxy_pass: bool,
        nginx_config: Option<Rc<NginxConfigHttpModel>>,
    },

    EditDomainMask(String),

    EditCfDomainRecord {
        domain: Rc<String>,
        proxied: bool,
        lb_ip: Rc<String>,
        cf_record_id: Option<String>,
    },

    SyncNginx {
        domain: Rc<String>,
        config: Rc<String>,
    },
}

pub enum DialogState {
    Hidden,
    Shown {
        header: String,
        dialog_type: DialogType,
    },
}

impl DialogState {
    pub fn show_dialog(&mut self, header: String, dialog_type: DialogType) {
        *self = Self::Shown {
            header,
            dialog_type,
        };
    }

    pub fn hide_dialog(&mut self) {
        *self = Self::Hidden;
    }

    pub fn as_ref(&self) -> &Self {
        self
    }
}
