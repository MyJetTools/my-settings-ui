use serde::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvsHttpResponse {
    pub name: String,
    pub envs: Vec<String>,
    pub prompt_ssh_pass_key: bool,
}
