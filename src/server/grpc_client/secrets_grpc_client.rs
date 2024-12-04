use my_grpc_extensions::client::*;
#[generate_grpc_client(
    proto_file: "./proto/SecretsService.proto",
    crate_ns: "crate::server::secrets_grpc",
    retries: 3,
    request_timeout_sec: 1,
    ping_timeout_sec: 1,
    ping_interval_sec: 3,
)]
pub struct SecretsGrpcClient;

/*
impl SecretsGrpcClient {
    pub async fn get_secret(name: String) -> Result<SecretModel, String> {
        let result = tokio::spawn(async move {
            let result = APP_CTX.secrets_grpc.get(GetSecretRequest { name }).await;

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
            let result = APP_CTX.secrets_grpc.get_all(()).await;

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
        let result = APP_CTX
            .secrets_grpc
            .save(SaveSecretRequest {
                model: Some(SecretModel { name, value, level }),
            })
            .await;

        match result {
            Ok(result) => Ok(result),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    pub async fn delete_secret(name: String) -> Result<(), String> {
        let result = APP_CTX
            .secrets_grpc
            .delete(DeleteSecretRequest { name })
            .await;

        match result {
            Ok(result) => Ok(result),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    pub async fn get_usage_of_templates(name: String) -> Result<Vec<TemplateUsageModel>, String> {
        let result = APP_CTX
            .secrets_grpc
            .get_templates_usage(GetTemplatesUsageRequest { name })
            .await;

        match result {
            Ok(result) => Ok(result.templates),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    pub async fn get_usage_of_secrets(name: String) -> Result<Vec<SecretUsageModel>, String> {
        let result = APP_CTX
            .secrets_grpc
            .get_secrets_usage(GetSecretsUsageRequest { name })
            .await;

        match result {
            Ok(result) => Ok(result.secrets),
            Err(err) => Err(format!("{:?}", err)),
        }
    }
}
 */
