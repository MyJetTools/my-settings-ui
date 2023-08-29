use flurl::IntoFlUrl;
use serde::Serialize;
#[derive(serde::Deserialize)]
pub struct SecretUsageBySecretModel {
    pub name: String,
    pub value: String,
}

#[derive(Serialize)]
struct RequestModel {
    name: String,
}

pub async fn get_secrets_usage(secret: String) -> Result<Vec<SecretUsageBySecretModel>, String> {
    let result = tokio::spawn(async move {
        let settings_reader = crate::APP_CTX.get_settings_reader().await;

        let url = settings_reader.get_url().await;

        let response = url
            .append_path_segment("api")
            .append_path_segment("secrets")
            .append_path_segment("usageBySecrets")
            .post_json(RequestModel { name: secret })
            .await;

        match response {
            Ok(mut response) => {
                let response: Vec<SecretUsageBySecretModel> = response.get_json().await.unwrap();
                Ok(response)
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
