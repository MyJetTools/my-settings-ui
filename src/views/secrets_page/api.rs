use dioxus::prelude::*;

use crate::models::*;

#[server]
pub async fn load_secrets(env_id: String) -> Result<Vec<SecretHttpModel>, ServerFnError> {
    use rust_extensions::date_time::DateTimeAsMicroseconds;
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    let response = ctx
        .secrets_grpc
        .get_all(())
        .await
        .unwrap()
        .into_vec()
        .await
        .unwrap();

    let result = response
        .into_iter()
        .map(|itm| SecretHttpModel {
            name: itm.name,
            level: itm.level,
            created: DateTimeAsMicroseconds::from_str(itm.created.as_str())
                .unwrap()
                .unix_microseconds,
            updated: DateTimeAsMicroseconds::from_str(itm.updated.as_str())
                .unwrap()
                .unix_microseconds,
            used_by_templates: itm.used_by_templates,
            used_by_secrets: itm.used_by_secrets,
        })
        .collect();

    Ok(result)
}

#[server]
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

#[server]
pub async fn delete_secret(env_id: String, secret_id: String) -> Result<(), ServerFnError> {
    use crate::server::secrets_grpc::*;
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    ctx.secrets_grpc
        .delete(DeleteSecretRequest { name: secret_id })
        .await
        .unwrap();

    Ok(())
}
