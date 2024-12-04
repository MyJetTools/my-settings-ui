use serde::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct CfDnsRecordRestApiModel {
    pub id: String,
    pub name: String,
    pub tp: String,
    pub content: String,
    pub proxied: bool,
}
