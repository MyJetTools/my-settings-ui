use serde::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SecretApiModel {
    pub secret_id: String,
    pub value: String,
    pub level: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SecretUsageBySecretApiModel {
    pub product_id: Option<String>,
    pub secret_id: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateUsageApiModel {
    pub product_id: String,
    pub template_id: String,
    pub yaml: String,
}
