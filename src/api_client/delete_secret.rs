use flurl::IntoFlUrl;
use serde::*;

#[derive(Serialize)]
struct DeleteSecretRequestModel {
    name: String,
}

pub async fn delete_secret(secret: String) -> Result<(), String> {
    let result = tokio::spawn(async move {
        let settings_reader = crate::APP_CTX.get_settings_reader().await;

        let url = settings_reader.get_url().await;

        let response = url
            .append_path_segment("api")
            .append_path_segment("secrets")
            .append_path_segment("delete")
            .post_json(DeleteSecretRequestModel {
                name: secret.clone(),
            })
            .await;

        match response {
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
