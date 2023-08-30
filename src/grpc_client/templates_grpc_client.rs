use my_telemetry::MyTelemetryContext;

use crate::APP_CTX;

#[my_grpc_client_macros::generate_grpc_client(
    proto_file: "./proto/TemplatesService.proto",
    crate_ns: "crate::templates_grpc",
    retries: 3,
    request_timeout_sec: 1,
    ping_timeout_sec: 1,
    ping_interval_sec: 3,
)]
pub struct TemplatesGrpcClient {
    channel: my_grpc_extensions::GrpcChannel<TGrpcService>,
}

impl TemplatesGrpcClient {
    pub async fn get_template(env: String, name: String) -> Result<String, String> {
        let result = tokio::spawn(async move {
            let grpc_client = APP_CTX.get_templates_grpc_client().await;

            let result = grpc_client
                .get(GetTemplateRequest { env, name }, &MyTelemetryContext::new())
                .await;

            match result {
                Ok(result) => Ok(result.yaml),
                Err(err) => Err(format!("{:?}", err)),
            }
        })
        .await;

        match result {
            Ok(result) => result,
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    pub async fn save_template(env: String, name: String, yaml: String) -> Result<(), String> {
        let result = tokio::spawn(async move {
            let grpc_client = APP_CTX.get_templates_grpc_client().await;

            let result = grpc_client
                .save(
                    SaveTemplateRequest { env, name, yaml },
                    &MyTelemetryContext::new(),
                )
                .await;

            match result {
                Ok(_) => Ok(()),
                Err(err) => Err(format!("{:?}", err)),
            }
        })
        .await;

        match result {
            Ok(result) => result,
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    pub async fn get_populated_template(env: String, name: String) -> Result<String, String> {
        let result = tokio::spawn(async move {
            let grpc_client = APP_CTX.get_templates_grpc_client().await;

            let result = grpc_client
                .compile_yaml(CompileYamlRequest { env, name }, &MyTelemetryContext::new())
                .await;

            match result {
                Ok(result) => Ok(result.yaml),
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
