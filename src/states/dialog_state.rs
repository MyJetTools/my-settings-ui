pub enum DialogType {
    ShowSecret(String),
    AddSecret,
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
