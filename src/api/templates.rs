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
        .into_vec()
        .await
        .unwrap();

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
    data: UpdateTemplateHttpModel,
) -> Result<(), ServerFnError> {
    use crate::server::templates_grpc::*;
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    ctx.templates_grpc
        .save(SaveTemplateRequest {
            env: data.env,
            name: data.name,
            yaml: data.yaml,
        })
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

#[server]
pub async fn get_template_content(
    env_id: String,
    env: String,
    name: String,
) -> Result<String, ServerFnError> {
    use crate::server::templates_grpc::*;
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    let response = ctx
        .templates_grpc
        .get(GetTemplateRequest { env, name })
        .await
        .unwrap();
    Ok(response.yaml)
}

#[server]
pub async fn download_snapshot(
    env_id: String,
    request: Vec<DownloadFileRequestModel>,
) -> Result<String, ServerFnError> {
    use crate::server::templates_grpc::GetTemplateRequest;
    use rust_extensions::base64::IntoBase64;
    let ctx = crate::server::APP_CTX.get_app_ctx(&env_id).await;

    let mut response = ctx.templates_grpc.get_all(()).await.unwrap();

    let mut result = Vec::new();
    while let Some(next_item) = response.get_next_item().await {
        let next_item = next_item.unwrap();

        if request
            .iter()
            .any(|itm| itm.env == next_item.env && itm.name == next_item.name)
        {
            let yaml = ctx
                .templates_grpc
                .get(GetTemplateRequest {
                    env: next_item.env.to_string(),
                    name: next_item.name.to_string(),
                })
                .await
                .unwrap();

            result.push(ExportItem {
                env: next_item.env,
                name: next_item.name,
                yaml: yaml.yaml.into_bytes().into_base64(),
            });
        }
    }

    Ok(serde_yaml::to_string(&result).unwrap())
}

#[server]
pub async fn upload_snapshot(env_id: String, snapshot: String) -> Result<(), ServerFnError> {
    use crate::server::templates_grpc::*;
    use rust_extensions::base64::*;

    let mut data: Vec<ExportItem> = serde_yaml::from_str(&snapshot).unwrap();

    for itm in data.iter_mut() {
        let data = itm.yaml.from_base64().unwrap();
        itm.yaml = String::from_utf8(data).unwrap();
    }

    let ctx = crate::server::APP_CTX.get_app_ctx(&env_id).await;

    for itm in data {
        ctx.templates_grpc
            .save(SaveTemplateRequest {
                env: itm.env,
                name: itm.name,
                yaml: itm.yaml,
            })
            .await
            .unwrap();
    }

    Ok(())
}

#[server]
pub async fn copy_template_to_other_env(
    from_env_id: String,
    to_env_id: String,
    env: String,
    name: String,
) -> Result<(), ServerFnError> {
    use crate::server::templates_grpc::*;
    let from_env_ctx = crate::server::APP_CTX
        .get_app_ctx(from_env_id.as_str())
        .await;

    let to_env_ctx = crate::server::APP_CTX.get_app_ctx(to_env_id.as_str()).await;

    let template_response = from_env_ctx
        .templates_grpc
        .get(GetTemplateRequest {
            env: env.to_string(),
            name: name.to_string(),
        })
        .await
        .unwrap();

    to_env_ctx
        .templates_grpc
        .save(SaveTemplateRequest {
            env: env,
            name: name,
            yaml: template_response.yaml,
        })
        .await
        .unwrap();

    Ok(())
}
