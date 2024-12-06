use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use crate::server::{grpc_client::*, settings::AppSettingsReader};

use super::SshPrivateKeyResolver;

pub struct AppCtxByEnv {
    pub templates_grpc: TemplatesGrpcClient,
    pub secrets_grpc: SecretsGrpcClient,
}

pub struct AppCtx {
    pub current_env: Mutex<HashMap<String, Arc<AppCtxByEnv>>>,
    pub app_settings_reader: Arc<AppSettingsReader>,
    pub private_key_resolver: Arc<SshPrivateKeyResolver>,
    //  pub domains_grpc_client: Arc<DomainsGrpcClient>,
    //pub settings: Arc<SettingsModel>,
}

impl AppCtx {
    pub fn new() -> Self {
        let app_settings_reader = Arc::new(AppSettingsReader::new());

        let private_key_resolver =
            Arc::new(SshPrivateKeyResolver::new(app_settings_reader.clone()));

        Self {
            private_key_resolver,
            current_env: Mutex::new(HashMap::new()),
            app_settings_reader,
            //   domains_grpc_client: Arc::new(DomainsGrpcClient::new(settings.clone())),
            //      settings,
        }
    }

    pub async fn get_app_ctx(&self, env_id: &str) -> Arc<AppCtxByEnv> {
        let mut write_access = self.current_env.lock().await;

        if let Some(ctx) = write_access.get(env_id) {
            return ctx.clone();
        }

        let env_app_settings = self.app_settings_reader.get_env_settings(env_id).await;

        let env_app_settings = Arc::new(env_app_settings);

        let templates_grpc = TemplatesGrpcClient::new(env_app_settings.clone());
        templates_grpc
            .set_ssh_private_key_resolver(self.private_key_resolver.clone())
            .await;

        let secrets_grpc = SecretsGrpcClient::new(env_app_settings.clone());
        secrets_grpc
            .set_ssh_private_key_resolver(self.private_key_resolver.clone())
            .await;

        let ctx = AppCtxByEnv {
            templates_grpc,
            secrets_grpc,
        };

        let ctx = Arc::new(ctx);

        write_access.insert(env_id.to_string(), ctx.clone());

        ctx
    }

    pub async fn get_envs(&self, user_id: &String) -> (Vec<String>, bool) {
        let settings = self.app_settings_reader.get_settings().await;

        let has_pass_phrase_entered = self.private_key_resolver.has_pass_phrase();

        if has_pass_phrase_entered {
            return (settings.get_envs(user_id), false);
        }

        (
            settings.get_envs(user_id),
            settings.prompt_ssh_pass_phrase.unwrap_or(false),
        )
    }
}
