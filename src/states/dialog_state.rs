pub enum DialogType {
    ShowSecret(String),
    AddSecret,
    EditSecret(String),
    DeleteSecret(String),

    AddTemplate,
    AddTemplateFromOtherTemplate { env: String, name: String },
    EditTemplate { env: String, name: String },
    DeleteTemplate { env: String, name: String },
    ShowPopulatedYaml { env: String, name: String },
    SecretUsage(String),
    SecretUsageBySecret(String),
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
