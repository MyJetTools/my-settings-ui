use serde::*;

use crate::grpc_client::{SecretsGrpcClient, TemplatesGrpcClient};

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    pub grpc_url: String,
    pub env_name: String,
}

impl SettingsReader {
    pub async fn get_env_name(&self) -> String {
        let read_access = self.settings.read().await;
        return read_access.env_name.clone();
    }
}

#[async_trait::async_trait]
impl my_grpc_extensions::GrpcClientSettings for SettingsReader {
    async fn get_grpc_url(&self, name: &'static str) -> String {
        if name == TemplatesGrpcClient::get_service_name() {
            let read_access = self.settings.read().await;
            return read_access.grpc_url.clone();
        }

        if name == SecretsGrpcClient::get_service_name() {
            let read_access = self.settings.read().await;
            return read_access.grpc_url.clone();
        }

        panic!("Unknown grpc service name: {}", name)
    }
}
