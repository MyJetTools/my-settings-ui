use std::sync::Arc;

use crate::{grpc_client::*, settings::SettingsModel};

pub struct AppCtx {
    pub templates_grpc: Arc<TemplatesGrpcClient>,
    pub secrets_grpc: Arc<SecretsGrpcClient>,
    pub settings: Arc<SettingsModel>,
}

impl AppCtx {
    pub fn new() -> Self {
        let settings = Arc::new(SettingsModel);
        Self {
            templates_grpc: Arc::new(TemplatesGrpcClient::new(settings.clone())),
            secrets_grpc: Arc::new(SecretsGrpcClient::new(settings.clone())),
            settings,
        }
    }
}
