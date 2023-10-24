use crate::views::{SecretListItemApiModel, TemplateApiModel};

pub enum MainState {
    Nothing,
    Templates(Option<Vec<TemplateApiModel>>),
    Secrets(Option<Vec<SecretListItemApiModel>>),
}

impl MainState {
    pub fn is_templates(&self) -> bool {
        match self {
            Self::Templates(_) => true,
            _ => false,
        }
    }

    pub fn is_secrets(&self) -> bool {
        match self {
            Self::Secrets(_) => true,
            _ => false,
        }
    }

    pub fn set_secrets(&mut self, secrets: Option<Vec<SecretListItemApiModel>>) {
        *self = Self::Secrets(secrets);
    }

    pub fn set_templates(&mut self, templates: Option<Vec<TemplateApiModel>>) {
        *self = Self::Templates(templates);
    }

    pub fn unwrap_as_templates(&self) -> &Option<Vec<TemplateApiModel>> {
        match self {
            Self::Templates(data) => data,
            _ => panic!("Not a templates state"),
        }
    }

    pub fn unwrap_as_secrets(&self) -> &Option<Vec<SecretListItemApiModel>> {
        match self {
            Self::Secrets(data) => data,
            _ => panic!("Not a templates state"),
        }
    }
}
