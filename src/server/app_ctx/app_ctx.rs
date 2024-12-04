use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use crate::server::{grpc_client::*, settings::AppSettingsReader};

pub struct AppCtxByEnv {
    pub templates_grpc: TemplatesGrpcClient,
    pub secrets_grpc: SecretsGrpcClient,
}

pub struct AppCtx {
    pub current_env: Mutex<HashMap<String, Arc<AppCtxByEnv>>>,
    //  pub domains_grpc_client: Arc<DomainsGrpcClient>,
    //pub settings: Arc<SettingsModel>,
}

impl AppCtx {
    pub fn new() -> Self {
        Self {
            current_env: Mutex::new(HashMap::new()),
            //   domains_grpc_client: Arc::new(DomainsGrpcClient::new(settings.clone())),
            //      settings,
        }
    }

    pub async fn get_app_ctx(&self, env_id: &str) -> Arc<AppCtxByEnv> {
        let mut write_access = self.current_env.lock().await;

        if let Some(ctx) = write_access.get(env_id) {
            return ctx.clone();
        }

        let app_settings = AppSettingsReader::new();

        let settings = app_settings.get_grpc_url_by_env(env_id).await;
        let settings = Arc::new(settings);

        let ctx = AppCtxByEnv {
            templates_grpc: TemplatesGrpcClient::new(settings.clone()),
            secrets_grpc: SecretsGrpcClient::new(settings),
        };

        let ctx = Arc::new(ctx);

        write_access.insert(env_id.to_string(), ctx.clone());

        ctx
    }

    pub async fn get_envs(&self) -> Vec<String> {
        let app_settings = AppSettingsReader::new();
        app_settings.get_envs().await
    }
}
