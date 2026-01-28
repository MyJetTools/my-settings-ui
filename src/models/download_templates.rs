use serde::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadFileRequestModel {
    pub product_id: String,
    pub template_id: String,
}
