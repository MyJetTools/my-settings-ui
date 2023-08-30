use flurl::IntoFlUrl;
use serde::*;

#[derive(Serialize)]
pub struct PostSecretModel {
    pub name: String,
    pub secret: String,
    pub level: i32,
}

pub async fn save_secret(name: String, value: String, level: i32) -> Result<(), String> {
    let result = tokio::spawn(async move {
        let settings_reader = crate::APP_CTX.get_settings_reader().await;

        let url = settings_reader.get_url().await;

        let response = url
            .append_path_segment("api")
            .append_path_segment("secrets")
            .append_path_segment("post")
            .post_json(PostSecretModel {
                name,
                secret: value,
                level,
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
