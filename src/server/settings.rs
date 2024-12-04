use std::collections::HashMap;

use serde::*;

use crate::server::grpc_client::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    pub envs: HashMap<String, String>,
}

pub struct AppSettingsReader {
    settings_reader: my_settings_reader::SettingsReader<SettingsModel>,
}

impl AppSettingsReader {
    pub fn new() -> Self {
        Self {
            settings_reader: my_settings_reader::SettingsReader::new_without_background_refresh(
                "~/.settings-ui",
            ),
        }
    }

    pub async fn get_grpc_url_by_env(&self, env: &str) -> AppGrpcClientSettings {
        let settings = self.settings_reader.get_settings().await;
        let url = settings.envs.get(env).unwrap();
        AppGrpcClientSettings(url.to_string())
    }

    pub async fn get_envs(&self) -> Vec<String> {
        let settings = self.settings_reader.get_settings().await;
        settings.envs.keys().map(|itm| itm.to_string()).collect()
    }
}

pub struct AppGrpcClientSettings(String);

#[async_trait::async_trait]
impl my_grpc_extensions::GrpcClientSettings for AppGrpcClientSettings {
    async fn get_grpc_url(&self, name: &'static str) -> String {
        if name == TemplatesGrpcClient::get_service_name() {
            return self.0.to_string();
        }

        if name == SecretsGrpcClient::get_service_name() {
            return self.0.to_string();
        }

        panic!("Unknown grpc service name: {}", name)
    }
}
