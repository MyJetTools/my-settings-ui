use std::rc::Rc;

use dioxus_utils::DataState;

use crate::{
    storage::ENV_LOCAL_STORAGE_KEY,
    views::{SecretListItemApiModel, TemplateApiModel},
};

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
    pub envs: DataState<Vec<Rc<String>>>,
    pub user: String,
    current_env_id: Rc<String>,
    pub location: LocationState,
    pub templates: DataState<Vec<TemplateApiModel>>,
    pub secrets: DataState<Vec<SecretListItemApiModel>>,
}

impl MainState {
    pub fn new(location: LocationState) -> Self {
        let current_env_id = dioxus_utils::js::GlobalAppSettings::get_local_storage()
            .get(ENV_LOCAL_STORAGE_KEY)
            .unwrap_or_default();
        Self {
            envs: DataState::None,
            location,
            templates: DataState::None,
            secrets: DataState::None,
            current_env_id: Rc::new(current_env_id),
            user: "".to_string(),
        }
    }

    pub fn set_envs(&mut self, envs: Vec<Rc<String>>) {
        let has_env = envs.iter().any(|env| env == &self.current_env_id);

        if !has_env {
            self.current_env_id = envs.first().unwrap().clone();
            dioxus_utils::js::GlobalAppSettings::get_local_storage()
                .set(ENV_LOCAL_STORAGE_KEY, &self.current_env_id);
        }

        self.envs = DataState::Loaded(envs);
    }

    pub fn get_selected_env(&self) -> Rc<String> {
        self.current_env_id.clone()
    }

    pub fn set_location(&mut self, location: LocationState) {
        self.location = location;
        self.drop_data();
    }

    pub fn active_env_changed(&mut self, value: &str) {
        dioxus_utils::js::GlobalAppSettings::get_local_storage().set(ENV_LOCAL_STORAGE_KEY, value);
        self.current_env_id = Rc::new(value.to_string());
        self.drop_data();
    }

    pub fn drop_data(&mut self) {
        self.templates = DataState::None;
        self.secrets = DataState::None;
    }
}
