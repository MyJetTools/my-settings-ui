use serde::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TemplateHttpModel {
    pub env: String,
    pub name: String,
    pub created: i64,
    pub updated: i64,
    pub last_requests: i64,
    pub has_missing_placeholders: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateTemplateHttpModel {
    pub env: String,
    pub name: String,
    pub yaml: String,
}
