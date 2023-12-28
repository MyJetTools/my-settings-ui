use serde::*;

use crate::grpc_client::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel;

impl SettingsModel {
    pub fn get_env_name(&self) -> String {
        read_env_variable("ENV_INFO")
    }

    pub fn get_cloud_flare_url(&self) -> String {
        read_env_variable("CLOUD_FLARE_URL")
    }

    pub fn get_nginx_api(&self) -> String {
        read_env_variable("NGINX_API")
    }
}

#[async_trait::async_trait]
impl my_grpc_extensions::GrpcClientSettings for SettingsModel {
    async fn get_grpc_url(&self, name: &'static str) -> String {
        if name == TemplatesGrpcClient::get_service_name() {
            return read_env_variable("SETTINGS_SERVICE_GRPC_URL");
        }

        if name == SecretsGrpcClient::get_service_name() {
            return read_env_variable("SETTINGS_SERVICE_GRPC_URL");
        }

        if name == DomainsGrpcClient::get_service_name() {
            return read_env_variable("SETTINGS_SERVICE_GRPC_URL");
        }

        panic!("Unknown grpc service name: {}", name)
    }
}

fn read_env_variable(name: &str) -> String {
    match std::env::var(name) {
        Ok(url) => return url,
        Err(_) => panic!("{} is not set", name),
    }
}
