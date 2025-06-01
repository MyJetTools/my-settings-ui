use serde::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SecretHttpModel {
    pub name: String,
    pub level: i32,
    pub created: i64,
    pub updated: i64,
    pub used_by_templates: i32,
    pub used_by_secrets: i32,
}
