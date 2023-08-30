use crate::{api_client::SecretModel, templates_grpc::TemplateListItem};

pub enum MainState {
    Nothing,
    Templates(Option<Vec<TemplateListItem>>),
    Secrets(Option<Vec<SecretModel>>),
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

    pub fn set_secrets(&mut self, secrets: Option<Vec<SecretModel>>) {
        *self = Self::Secrets(secrets);
    }

    pub fn set_templates(&mut self, templates: Option<Vec<TemplateListItem>>) {
        *self = Self::Templates(templates);
    }

    pub fn unwrap_as_templates(&self) -> &Option<Vec<TemplateListItem>> {
        match self {
            Self::Templates(data) => data,
            _ => panic!("Not a templates state"),
        }
    }

    pub fn unwrap_as_secrets(&self) -> &Option<Vec<SecretModel>> {
        match self {
            Self::Secrets(data) => data,
            _ => panic!("Not a templates state"),
        }
    }
}
