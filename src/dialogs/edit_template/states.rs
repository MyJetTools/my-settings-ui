use std::rc::Rc;

use dioxus_utils::*;

use crate::models::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditTemplateDialogData {
    New,
    Edit(Rc<TemplateHttpModel>),
    CopyFromOtherTemplate(Rc<TemplateHttpModel>),
}

pub struct EditTemplateState {
    pub tabs: EditTemplateTab,
    pub env_id: Rc<String>,
    pub env: DialogValue<String>,
    pub name: DialogValue<String>,
    pub yaml: DialogValue<String>,
    pub init_data: EditTemplateDialogData,
    pub init_from_other_template: Option<LoadDataFromTemplate>,
}

impl EditTemplateState {
    pub fn new(env_id: Rc<String>, data: EditTemplateDialogData) -> Self {
        match &data {
            EditTemplateDialogData::New => Self {
                tabs: Default::default(),
                env_id,
                env: Default::default(),
                name: Default::default(),
                yaml: Default::default(),
                init_data: data,
                init_from_other_template: None,
            },
            EditTemplateDialogData::Edit(template) => Self {
                init_from_other_template: Some(LoadDataFromTemplate {
                    src_template: template.clone(),
                    init_status: Default::default(),
                }),
                tabs: Default::default(),
                env_id,
                env: DialogValue::new(template.env.to_string()),
                name: DialogValue::new(template.name.to_string()),
                yaml: Default::default(),
                init_data: data,
            },
            EditTemplateDialogData::CopyFromOtherTemplate(template) => Self {
                init_from_other_template: Some(LoadDataFromTemplate {
                    src_template: template.clone(),
                    init_status: Default::default(),
                }),
                tabs: Default::default(),
                env_id,
                env: DialogValue::new(template.env.to_string()),
                name: DialogValue::new(template.name.to_string()),
                yaml: Default::default(),
                init_data: data,
            },
        }
    }

    pub fn save_button_disabled(&self) -> bool {
        return !self.name.is_value_updated()
            && !self.yaml.is_value_updated()
            && !self.env.is_value_updated();
    }

    pub fn is_new_template(&self) -> bool {
        match self.init_data {
            EditTemplateDialogData::New => true,
            _ => false,
        }
    }

    pub fn add_secret_to_yaml(&mut self, value: &str) {
        let value_access = self.yaml.get_value_mut();
        value_access.push_str("${");
        value_access.push_str(value);
        value_access.push('}');
    }

    pub fn unwrap_into_http_model(&self) -> UpdateTemplateHttpModel {
        UpdateTemplateHttpModel {
            env: self.env.get_value().to_string(),
            name: self.name.get_value().to_string(),
            yaml: self.yaml.get_value().to_string(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum EditTemplateTab {
    ChooseSecret,
    PeekSecret,
}

impl Default for EditTemplateTab {
    fn default() -> Self {
        Self::ChooseSecret
    }
}

pub struct LoadDataFromTemplate {
    pub src_template: Rc<TemplateHttpModel>,
    pub init_status: DataState<()>,
}
