use dioxus::prelude::*;

use crate::models::*;

#[get("/api/secrets/load?env_id&product_id")]
pub async fn load_secrets(
    env_id: String,
    product_id: String,
) -> Result<Vec<SecretHttpModel>, ServerFnError> {
    use crate::server::secrets_grpc::*;
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    let result: Vec<SecretHttpModel> = ctx
        .secrets_grpc
        .get_all(GetAllSecretsGrpcRequest {
            product_id,
            include_shared: true,
        })
        .await
        .unwrap()
        .into_vec()
        .await
        .unwrap();

    Ok(result)
}

#[post("/api/secrets/save")]
pub async fn save_secret(
    env_id: String,
    value: UpdateSecretValueHttpModel,
) -> Result<(), ServerFnError> {
    use crate::server::secrets_grpc::*;
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    ctx.secrets_grpc
        .save(SaveSecretGrpcRequest {
            product_id: value.product_id,
            id: value.secret_id,
            value: value.value,
            level: value.level,
        })
        .await
        .unwrap();

    Ok(())
}

#[post("/api/secrets/delete")]
pub async fn delete_secret(
    env_id: String,
    product_id: Option<String>,
    secret_id: String,
) -> Result<(), ServerFnError> {
    use crate::server::secrets_grpc::*;
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    ctx.secrets_grpc
        .delete(DeleteSecretGrpcRequest {
            secret_id,
            product_id,
        })
        .await
        .unwrap();

    Ok(())
}

#[get("/api/secrets/load_one?env_id&product_id&secret_id")]
pub async fn load_secret(
    env_id: String,
    product_id: Option<String>,
    secret_id: String,
) -> Result<SecretApiModel, ServerFnError> {
    use crate::server::secrets_grpc::*;
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    let response = ctx
        .secrets_grpc
        .get(GetSecretGrpcRequest {
            secret_id: secret_id.to_string(),
            product_id,
        })
        .await
        .unwrap();

    let result = SecretApiModel {
        secret_id: secret_id,
        value: response.value,
        level: response.level,
    };

    Ok(result)
}

#[post("/api/secrets/copy_to_other_env")]
pub async fn copy_secret_to_other_env(
    from_env_id: String,
    to_env_id: String,
    product_id: Option<String>,
    secret_id: String,
) -> Result<(), ServerFnError> {
    use crate::server::secrets_grpc::*;
    let from_env_ctx = crate::server::APP_CTX
        .get_app_ctx(from_env_id.as_str())
        .await;

    let to_env_ctx = crate::server::APP_CTX.get_app_ctx(to_env_id.as_str()).await;

    let secret_model = from_env_ctx
        .secrets_grpc
        .get(GetSecretGrpcRequest {
            secret_id: secret_id.clone(),
            product_id: product_id.clone(),
        })
        .await
        .unwrap();

    to_env_ctx
        .secrets_grpc
        .save(SaveSecretGrpcRequest {
            product_id,
            id: secret_id,
            value: secret_model.value,
            level: secret_model.level,
        })
        .await
        .unwrap();

    Ok(())
}

#[get("/api/secrets/load_secret_value?env_id&product_id&secret_id")]
pub async fn load_secret_value(
    env_id: String,
    product_id: Option<String>,
    secret_id: String,
) -> Result<SecretValueApiModel, ServerFnError> {
    use crate::server::secrets_grpc::*;
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    let response = ctx
        .secrets_grpc
        .get(GetSecretGrpcRequest {
            product_id,
            secret_id,
        })
        .await
        .unwrap();

    let result = SecretValueApiModel {
        value: response.value,
        level: response.level,
    };

    Ok(result)
}

#[get("/api/secrets/get_secret_usage_by_secret?env_id&product_id&secret_id")]
pub async fn load_secret_usage_by_secret(
    env_id: String,
    product_id: Option<String>,
    secret_id: String,
) -> Result<Vec<SecretUsageBySecretApiModel>, ServerFnError> {
    use crate::server::secrets_grpc::*;
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    let response = ctx
        .secrets_grpc
        .get_secrets_usage(DeleteSecretGrpcRequest {
            secret_id,
            product_id,
        })
        .await
        .unwrap();

    let result: Vec<_> = response
        .secrets
        .into_iter()
        .map(|itm| SecretUsageBySecretApiModel {
            product_id: if itm.product_id.len() == 0 {
                None
            } else {
                Some(itm.product_id)
            },
            secret_id: itm.id,
            value: itm.value,
        })
        .collect();

    Ok(result)
}

#[get("/api/secrets/load_secret_usage_by_templates?env_id&product_id&secret_id")]
pub async fn load_secret_usage_by_templates(
    env_id: String,
    product_id: Option<String>,
    secret_id: String,
) -> Result<Vec<TemplateUsageApiModel>, ServerFnError> {
    use crate::server::secrets_grpc::*;
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    let response = ctx
        .secrets_grpc
        .get_templates_usage(GetTemplatesUsageGrpcRequest {
            product_id,
            secret_id,
        })
        .await
        .unwrap();

    let result: Vec<TemplateUsageApiModel> = response
        .templates
        .into_iter()
        .map(|itm| TemplateUsageApiModel {
            product_id: itm.product,
            template_id: itm.template_id,
            yaml: itm.template_content,
        })
        .collect();

    Ok(result)
}

#[cfg(feature = "server")]
impl From<crate::server::secrets_grpc::SecretGrpcModel> for SecretHttpModel {
    fn from(item: crate::server::secrets_grpc::SecretGrpcModel) -> Self {
        SecretHttpModel {
            product_id: item.product_id,
            secret_id: item.secret_id,
            level: item.level,
            created: rust_extensions::date_time::DateTimeAsMicroseconds::from_str(
                item.created.as_str(),
            )
            .unwrap()
            .unix_microseconds,
            updated: rust_extensions::date_time::DateTimeAsMicroseconds::from_str(
                item.updated.as_str(),
            )
            .unwrap()
            .unix_microseconds,
            used_by_templates: item.used_by_templates,
            used_by_secrets: item.used_by_secrets,
        }
    }
}
