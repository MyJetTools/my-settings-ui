use std::collections::HashMap;

use my_ssh::SshCredentialsSettingsModel;
use serde::*;

use crate::server::grpc_client::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    pub envs: HashMap<String, String>,
    pub ssh_private_keys: Option<HashMap<String, SshCredentialsSettingsModel>>,
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

    pub async fn get_env_settings(&self, env: &str) -> EnvSettings {
        let settings = self.settings_reader.get_settings().await;
        if let Some(result) = settings.envs.get(env) {
            return EnvSettings {
                url: result.to_string(),
                ssh_private_keys: settings.ssh_private_keys.clone(),
            };
        }

        panic!("Can not get settings for env: '{}'", env);
    }

    pub async fn get_envs(&self) -> Vec<String> {
        let settings = self.settings_reader.get_settings().await;
        settings.envs.keys().map(|itm| itm.to_string()).collect()
    }
}

pub struct EnvSettings {
    url: String,
    ssh_private_keys: Option<HashMap<String, SshCredentialsSettingsModel>>,
}

#[async_trait::async_trait]
impl my_grpc_extensions::GrpcClientSettings for EnvSettings {
    async fn get_grpc_url(&self, name: &'static str) -> String {
        if name == TemplatesGrpcClient::get_service_name() {
            return self.url.to_string();
        }

        if name == SecretsGrpcClient::get_service_name() {
            return self.url.to_string();
        }

        panic!("Unknown grpc service name: {}", name)
    }
}

#[async_trait::async_trait]
impl my_ssh::SshPrivateKeyResolver for EnvSettings {
    async fn resolve_ssh_private_key(&self, ssh_line: &str) -> Option<my_ssh::SshPrivateKey> {
        let private_keys = self.ssh_private_keys.as_ref()?;

        if let Some(ssh_credentials) = private_keys.get(ssh_line) {
            return my_ssh::SshPrivateKey {
                content: ssh_credentials.load_cert().await,
                pass_phrase: ssh_credentials.cert_pass_phrase.clone(),
            }
            .into();
        }

        if let Some(ssh_credentials) = private_keys.get("*") {
            return my_ssh::SshPrivateKey {
                content: ssh_credentials.load_cert().await,
                pass_phrase: ssh_credentials.cert_pass_phrase.clone(),
            }
            .into();
        }

        None
    }
}
