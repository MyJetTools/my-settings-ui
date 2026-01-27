use serde::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TemplateHttpModel {
    pub product_id: String,
    pub template_id: String,
    pub created: i64,
    pub updated: i64,
    pub last_requests: i64,
    pub has_missing_placeholders: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateTemplateHttpModel {
    pub product_id: String,
    pub template_id: String,
    pub yaml: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PopulatedYamlModelApiModel {
    pub yaml: String,
}
