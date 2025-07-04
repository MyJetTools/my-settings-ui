use serde::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadFileRequestModel {
    pub env: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExportItem {
    pub env: String,
    pub name: String,
    pub yaml: String,
}
