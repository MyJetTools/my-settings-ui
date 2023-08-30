use flurl::IntoFlUrl;
use serde::*;

#[derive(Serialize)]
struct RequestModel {
    name: String,
}

#[derive(Deserialize)]
pub struct SecretValueModel {
    pub value: String,
    pub level: i32,
}

pub async fn load_secret(secret: String) -> Result<SecretValueModel, String> {
    let result = tokio::spawn(async move {
        let settings_reader = crate::APP_CTX.get_settings_reader().await;

        let url = settings_reader.get_url().await;

        let response = url
            .append_path_segment("api")
            .append_path_segment("secrets")
            .append_path_segment("get")
            .post_json(RequestModel {
                name: secret.clone(),
            })
            .await;

        match response {
            Ok(mut response) => {
                let response: SecretValueModel = response.get_json().await.unwrap();
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
