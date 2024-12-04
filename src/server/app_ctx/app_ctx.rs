use std::sync::Arc;

use crate::server::{grpc_client::*, settings::AppSettingsReader};

pub struct AppCtx {
    pub templates_grpc: Arc<TemplatesGrpcClient>,
    pub secrets_grpc: Arc<SecretsGrpcClient>,
    //  pub domains_grpc_client: Arc<DomainsGrpcClient>,
    //pub settings: Arc<SettingsModel>,
}

impl AppCtx {
    pub fn new() -> Self {
        let settings_reader = AppSettingsReader::new();
        let settings_reader = Arc::new(settings_reader);
        Self {
            templates_grpc: Arc::new(TemplatesGrpcClient::new(settings_reader.clone())),
            secrets_grpc: Arc::new(SecretsGrpcClient::new(settings_reader.clone())),
            //   domains_grpc_client: Arc::new(DomainsGrpcClient::new(settings.clone())),
            //      settings,
        }
    }
}
