use serde::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SecretApiModel {
    pub name: String,
    pub value: String,
    pub level: i32,
}
