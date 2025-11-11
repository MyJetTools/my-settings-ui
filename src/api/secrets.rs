use dioxus::prelude::*;

use crate::models::*;

#[get("/api/secrets/load?env_id")]
pub async fn load_secrets(env_id: String) -> Result<Vec<SecretHttpModel>, ServerFnError> {
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    let result: Vec<SecretHttpModel> = ctx
        .secrets_grpc
        .get_all(())
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
    name: String,
    value: String,
    level: i32,
) -> Result<(), ServerFnError> {
    use crate::server::secrets_grpc::*;
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    ctx.secrets_grpc
        .save(SaveSecretRequest {
            model: SecretModel { name, value, level }.into(),
        })
        .await
        .unwrap();

    Ok(())
}

#[post("/api/secrets/delete")]
pub async fn delete_secret(env_id: String, secret_id: String) -> Result<(), ServerFnError> {
    use crate::server::secrets_grpc::*;
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    ctx.secrets_grpc
        .delete(DeleteSecretRequest { name: secret_id })
        .await
        .unwrap();

    Ok(())
}

#[get("/api/secrets/load_one?env_id&secret_name")]
pub async fn load_secret(
    env_id: String,
    secret_name: String,
) -> Result<SecretApiModel, ServerFnError> {
    use crate::server::secrets_grpc::*;
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    let response = ctx
        .secrets_grpc
        .get(GetSecretRequest {
            name: secret_name.to_string(),
        })
        .await
        .unwrap();

    let result = SecretApiModel {
        name: secret_name,
        value: response.value,
        level: response.level,
    };

    Ok(result)
}

#[post("/api/secrets/copy_to_other_env")]
pub async fn copy_secret_to_other_env(
    from_env_id: String,
    to_env_id: String,
    secret_id: String,
) -> Result<(), ServerFnError> {
    use crate::server::secrets_grpc::*;
    let from_env_ctx = crate::server::APP_CTX
        .get_app_ctx(from_env_id.as_str())
        .await;

    let to_env_ctx = crate::server::APP_CTX.get_app_ctx(to_env_id.as_str()).await;

    let secret_model = from_env_ctx
        .secrets_grpc
        .get(GetSecretRequest {
            name: secret_id.to_string(),
        })
        .await
        .unwrap();

    to_env_ctx
        .secrets_grpc
        .save(SaveSecretRequest {
            model: Some(secret_model),
        })
        .await
        .unwrap();

    Ok(())
}

#[cfg(feature = "server")]
impl From<crate::server::secrets_grpc::SecretListItem> for SecretHttpModel {
    fn from(item: crate::server::secrets_grpc::SecretListItem) -> Self {
        SecretHttpModel {
            name: item.name,
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
