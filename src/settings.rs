use serde::*;

use crate::grpc_client::TemplatesGrpcClient;

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    pub url: String,
    pub grpc_url: String,
}

impl SettingsReader {
    pub async fn get_url(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.url.clone()
    }
}

#[async_trait::async_trait]
impl my_grpc_extensions::GrpcClientSettings for SettingsReader {
    async fn get_grpc_url(&self, name: &'static str) -> String {
        if name == TemplatesGrpcClient::get_service_name() {
            let read_access = self.settings.read().await;
            return read_access.grpc_url.clone();
        }

        panic!("Unknown grpc service name: {}", name)
    }
}
