use dioxus_utils::DataState;

use crate::views::{SecretListItemApiModel, TemplateApiModel};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LocationState {
    None,
    Templates,
    Secrets,
}

impl LocationState {
    pub fn is_templates(&self) -> bool {
        match self {
            Self::Templates => true,
            _ => false,
        }
    }

    pub fn is_secrets(&self) -> bool {
        match self {
            Self::Secrets => true,
            _ => false,
        }
    }
}

pub struct MainState {
    pub location: LocationState,
    pub templates: DataState<Vec<TemplateApiModel>>,
    pub secrets: DataState<Vec<SecretListItemApiModel>>,
}

impl MainState {
    pub fn new(location: LocationState) -> Self {
        Self {
            location,
            templates: DataState::None,
            secrets: DataState::None,
        }
    }

    pub fn set_location(&mut self, location: LocationState) {
        self.location = location;
        self.drop_data();
    }

    pub fn drop_data(&mut self) {
        self.templates = DataState::None;
        self.secrets = DataState::None;
    }
}
