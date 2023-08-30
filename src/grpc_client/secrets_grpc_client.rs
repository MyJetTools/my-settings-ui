use my_telemetry::MyTelemetryContext;

use crate::APP_CTX;

#[my_grpc_client_macros::generate_grpc_client(
    proto_file: "./proto/SecretsService.proto",
    crate_ns: "crate::secrets_grpc",
    retries: 3,
    request_timeout_sec: 1,
    ping_timeout_sec: 1,
    ping_interval_sec: 3,
)]
pub struct SecretsGrpcClient {
    channel: my_grpc_extensions::GrpcChannel<TGrpcService>,
}

impl SecretsGrpcClient {
    pub async fn get_secret(name: String) -> Result<SecretModel, String> {
        let result = tokio::spawn(async move {
            let grpc_client = APP_CTX.get_secrets_grpc_client().await;

            let result = grpc_client
                .get(GetSecretRequest { name }, &MyTelemetryContext::new())
                .await;

            match result {
                Ok(result) => Ok(result),
                Err(err) => Err(format!("{:?}", err)),
            }
        })
        .await;

        match result {
            Ok(result) => result,
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    pub async fn get_all_secrets() -> Result<Vec<SecretListItem>, String> {
        let result = tokio::spawn(async move {
            let grpc_client = APP_CTX.get_secrets_grpc_client().await;

            let result = grpc_client.get_all((), &MyTelemetryContext::new()).await;

            match result {
                Ok(result) => match result {
                    Some(result) => Ok(result),
                    None => Ok(vec![]),
                },
                Err(err) => Err(format!("{:?}", err)),
            }
        })
        .await;

        match result {
            Ok(result) => result,
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    pub async fn save_secret(name: String, value: String, level: i32) -> Result<(), String> {
        let result = tokio::spawn(async move {
            let grpc_client = APP_CTX.get_secrets_grpc_client().await;

            let result = grpc_client
                .save(
                    SaveSecretRequest {
                        model: Some(SecretModel { name, value, level }),
                    },
                    &MyTelemetryContext::new(),
                )
                .await;

            match result {
                Ok(result) => Ok(result),
                Err(err) => Err(format!("{:?}", err)),
            }
        })
        .await;

        match result {
            Ok(result) => result,
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    pub async fn delete_secret(name: String) -> Result<(), String> {
        let result = tokio::spawn(async move {
            let grpc_client = APP_CTX.get_secrets_grpc_client().await;

            let result = grpc_client
                .delete(DeleteSecretRequest { name }, &MyTelemetryContext::new())
                .await;

            match result {
                Ok(result) => Ok(result),
                Err(err) => Err(format!("{:?}", err)),
            }
        })
        .await;

        match result {
            Ok(result) => result,
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    pub async fn get_usage_of_templates(name: String) -> Result<Vec<TemplateUsageModel>, String> {
        let result = tokio::spawn(async move {
            let grpc_client = APP_CTX.get_secrets_grpc_client().await;

            let result = grpc_client
                .get_templates_usage(
                    GetTemplatesUsageRequest { name },
                    &MyTelemetryContext::new(),
                )
                .await;

            match result {
                Ok(result) => Ok(result.templates),
                Err(err) => Err(format!("{:?}", err)),
            }
        })
        .await;

        match result {
            Ok(result) => result,
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    pub async fn get_usage_of_secrets(name: String) -> Result<Vec<SecretUsageModel>, String> {
        let result = tokio::spawn(async move {
            let grpc_client = APP_CTX.get_secrets_grpc_client().await;

            let result = grpc_client
                .get_secrets_usage(GetSecretsUsageRequest { name }, &MyTelemetryContext::new())
                .await;

            match result {
                Ok(result) => Ok(result.secrets),
                Err(err) => Err(format!("{:?}", err)),
            }
        })
        .await;

        match result {
            Ok(result) => result,
            Err(err) => Err(format!("{:?}", err)),
        }
    }
}
