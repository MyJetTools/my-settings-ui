use dioxus::prelude::*;

use crate::models::*;

#[server]
pub async fn get_templates(env_id: String) -> Result<Vec<TemplateHttpModel>, ServerFnError> {
    use rust_extensions::date_time::DateTimeAsMicroseconds;
    use std::collections::BTreeMap;

    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    let response = ctx
        .templates_grpc
        .get_all(())
        .await
        .unwrap()
        .unwrap_or_default();

    let result: BTreeMap<_, _> = response
        .into_iter()
        .map(|itm| {
            (
                format!("{}/{}", itm.name, itm.env),
                TemplateHttpModel {
                    env: itm.env,
                    name: itm.name,
                    created: match DateTimeAsMicroseconds::from_str(itm.created.as_str()) {
                        Some(itm) => itm.unix_microseconds,
                        None => 0,
                    },
                    updated: match DateTimeAsMicroseconds::from_str(itm.updated.as_str()) {
                        Some(itm) => itm.unix_microseconds,
                        None => 0,
                    },
                    last_requests: itm.last_requests,
                    has_missing_placeholders: itm.has_missing_placeholders,
                },
            )
        })
        .collect();

    let result = result.into_iter().map(|itm| itm.1).collect();

    Ok(result)
}

#[server]
pub async fn save_template(
    env_id: String,
    env: String,
    name: String,
    yaml: String,
) -> Result<(), ServerFnError> {
    use crate::server::templates_grpc::*;
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    ctx.templates_grpc
        .save(SaveTemplateRequest { env, name, yaml })
        .await
        .unwrap();

    Ok(())
}

#[server]
pub async fn delete_template(
    env_id: String,
    env: String,
    name: String,
) -> Result<(), ServerFnError> {
    use crate::server::templates_grpc::*;
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    ctx.templates_grpc
        .delete(DeleteTemplateRequest { env, name })
        .await
        .unwrap();

    Ok(())
}
