use flurl::IntoFlUrl;
use serde::Serialize;
#[derive(serde::Deserialize)]
struct SecretUsageContract {
    pub data: Vec<SecretUsageModel>,
}

#[derive(serde::Deserialize, Clone)]
pub struct SecretUsageModel {
    pub env: String,
    pub name: String,
    pub yaml: String,
}

#[derive(Serialize)]
struct RequestModel {
    name: String,
}

pub async fn get_templates_usage(secret: String) -> Result<Vec<SecretUsageModel>, String> {
    let result: Result<Result<Vec<SecretUsageModel>, String>, tokio::task::JoinError> =
        tokio::spawn(async move {
            let settings_reader = crate::APP_CTX.get_settings_reader().await;

            let url = settings_reader.get_url().await;

            let response = url
                .append_path_segment("api")
                .append_path_segment("secrets")
                .append_path_segment("usage")
                .post_json(RequestModel { name: secret })
                .await;

            match response {
                Ok(mut response) => {
                    let response: SecretUsageContract = response.get_json().await.unwrap();
                    Ok(response.data)
                }
                Err(err) => Err(format!("{:?}", err)),
            }
        })
        .await;

    match result {
        Ok(result) => result,
        Err(err) => Err(format!("{:?}", err)),
    }
}
