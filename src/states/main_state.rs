use std::rc::Rc;

use dioxus_utils::DataState;

use crate::{models::*, storage::*};

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
    selected_product_id: Option<Rc<String>>,
    pub location: LocationState,
    pub templates: DataState<Vec<Rc<TemplateHttpModel>>>,
    pub secrets: DataState<Vec<SecretHttpModel>>,
    pub prompt_ssh_key: Option<bool>,
}

impl MainState {
    pub fn new(location: LocationState) -> Self {
        let current_env_id = dioxus_utils::js::GlobalAppSettings::get_local_storage()
            .get(ENV_LOCAL_STORAGE_KEY)
            .unwrap_or_default();

        let selected_product_id = dioxus_utils::js::GlobalAppSettings::get_local_storage()
            .get(PRODUCT_ID_LOCAL_STORAGE_KEY);

        Self {
            envs: DataState::default(),
            location,
            templates: DataState::default(),
            secrets: DataState::default(),
            current_env_id: Rc::new(current_env_id),
            selected_product_id: selected_product_id.map(Rc::new),
            user: "".to_string(),
            prompt_ssh_key: None,
        }
    }

    pub fn set_envs(&mut self, envs: Vec<Rc<String>>) {
        let has_env = envs.iter().any(|env| env == &self.current_env_id);

        if !has_env {
            self.current_env_id = envs.first().unwrap().clone();
            dioxus_utils::js::GlobalAppSettings::get_local_storage()
                .set(ENV_LOCAL_STORAGE_KEY, &self.current_env_id);
        }

        self.envs.set_loaded(envs);
    }

    pub fn get_selected_env(&self) -> Rc<String> {
        self.current_env_id.clone()
    }

    pub fn get_selected_product_id(&self) -> Option<Rc<String>> {
        self.selected_product_id.clone()
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
        self.templates.reset();
        self.secrets.reset();
    }
}
