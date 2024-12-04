use serde::*;

use crate::server::grpc_client::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    pub settings_grpc_url: String,
}

pub struct AppSettingsReader {
    settings_reader: my_settings_reader::SettingsReader<SettingsModel>,
}

impl AppSettingsReader {
    pub fn new() -> Self {
        Self {
            settings_reader: my_settings_reader::SettingsReader::new("~/.settings-ui"),
        }
    }
}

#[async_trait::async_trait]
impl my_grpc_extensions::GrpcClientSettings for AppSettingsReader {
    async fn get_grpc_url(&self, name: &'static str) -> String {
        if name == TemplatesGrpcClient::get_service_name() {
            return self
                .settings_reader
                .get(|settings| settings.settings_grpc_url.clone())
                .await;
        }

        if name == SecretsGrpcClient::get_service_name() {
            return self
                .settings_reader
                .get(|settings| settings.settings_grpc_url.clone())
                .await;
        }

        panic!("Unknown grpc service name: {}", name)
    }
}
