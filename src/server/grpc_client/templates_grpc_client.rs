use my_grpc_extensions::client::*;

#[generate_grpc_client(
    proto_file: "./proto/TemplatesService.proto",
    crate_ns: "crate::server::templates_grpc",
    retries: 3,
    request_timeout_sec: 1,
    ping_timeout_sec: 1,
    ping_interval_sec: 3,
)]
pub struct TemplatesGrpcClient;

/*
impl TemplatesGrpcClient {
    pub async fn get_template(env: String, name: String) -> Result<String, String> {
        let result = APP_CTX
            .templates_grpc
            .get(GetTemplateRequest { env, name })
            .await;

        match result {
            Ok(result) => Ok(result.yaml),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    pub async fn get_all_templates() -> Result<Vec<TemplateListItem>, String> {
        let result = APP_CTX.templates_grpc.get_all(()).await;

        match result {
            Ok(result) => match result {
                Some(result) => Ok(result),
                None => Ok(vec![]),
            },
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    pub async fn save_template(env: String, name: String, yaml: String) -> Result<(), String> {
        let result = APP_CTX
            .templates_grpc
            .save(SaveTemplateRequest { env, name, yaml })
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    pub async fn delete_template(env: String, name: String) -> Result<(), String> {
        let result = APP_CTX
            .templates_grpc
            .delete(DeleteTemplateRequest { env, name })
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("{:?}", err)),
        }
    }
    pub async fn get_populated_template(env: String, name: String) -> Result<String, String> {
        let result = APP_CTX
            .templates_grpc
            .compile_yaml(CompileYamlRequest { env, name })
            .await;

        match result {
            Ok(result) => Ok(result.yaml),
            Err(err) => Err(format!("{:?}", err)),
        }
    }
}
 */
