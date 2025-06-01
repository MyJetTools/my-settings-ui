use std::{collections::HashMap, sync::Arc};

use serde::*;

use crate::server::grpc_client::*;
use my_ssh::ssh_settings::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    pub envs: HashMap<String, EnvSettingsModel>,
    pub ssh_private_keys: Option<HashMap<String, SshPrivateKeySettingsModel>>,
    pub users: Option<HashMap<String, Vec<String>>>,
    pub prompt_ssh_pass_phrase: Option<bool>,
}

impl SettingsModel {
    fn has_user(&self, user_group: &str, user_id: &str) -> bool {
        if self.users.is_none() {
            return false;
        }

        if let Some(users) = self.users.as_ref().unwrap().get(user_group) {
            for user in users {
                if user == user_id {
                    return true;
                }
            }
        }

        false
    }

    pub fn get_envs(&self, user_id: &String) -> Vec<String> {
        let mut result = Vec::new();

        for (env, env_settings) in self.envs.iter() {
            if env_settings.users == "*" {
                result.push(env.clone());
                continue;
            }

            if self.has_user(&env_settings.users, user_id) {
                result.push(env.clone());
            }
        }

        result
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnvSettingsModel {
    pub url: String,
    pub users: String,
    pub host: Option<String>,
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
                url: result.url.to_string(),
                host: result.host.clone(),
            };
        }

        panic!("Can not get settings for env: '{}'", env);
    }

    pub async fn get_settings(&self) -> Arc<SettingsModel> {
        self.settings_reader.get_settings().await
    }
}

pub struct EnvSettings {
    url: String,
    host: Option<String>,
}

#[async_trait::async_trait]
impl my_grpc_extensions::GrpcClientSettings for EnvSettings {
    async fn get_grpc_url(&self, name: &'static str) -> my_grpc_extensions::GrpcUrl {
        if name == TemplatesGrpcClient::get_service_name() {
            return my_grpc_extensions::GrpcUrl {
                url: self.url.to_string(),
                host_metadata: self.host.clone(),
            };
        }

        if name == SecretsGrpcClient::get_service_name() {
            return my_grpc_extensions::GrpcUrl {
                url: self.url.to_string(),
                host_metadata: self.host.clone(),
            };
        }

        panic!("Unknown grpc service name: {}", name)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::SettingsModel;

    #[test]
    fn test() {
        let mut users = HashMap::new();
        users.insert(
            "Group1".to_string(),
            vec!["User1".to_string(), "User2".to_string()],
        );
        users.insert(
            "Group2".to_string(),
            vec!["User3".to_string(), "User4".to_string()],
        );

        let mut envs = HashMap::new();

        envs.insert(
            "env1".to_string(),
            super::EnvSettingsModel {
                url: "http://localhost:5000".to_string(),
                users: "Group1".to_string(),
                host: Default::default(),
            },
        );

        envs.insert(
            "env2".to_string(),
            super::EnvSettingsModel {
                url: "http://localhost:5001".to_string(),
                users: "Group2".to_string(),
                host: Default::default(),
            },
        );

        let mut ssh_private_keys = HashMap::new();
        ssh_private_keys.insert(
            "env1".to_string(),
            my_ssh::ssh_settings::SshPrivateKeySettingsModel {
                cert_path: "~/.ssh/id_rsa".to_string(),
                cert_pass_phrase: "pass".to_string().into(),
            },
        );

        let model = SettingsModel {
            envs,
            ssh_private_keys: Some(ssh_private_keys),
            users: Some(users),
            prompt_ssh_pass_phrase: Some(true),
        };

        let result = serde_yaml::to_string(&model).unwrap();

        println!("{}", result);
    }
}
