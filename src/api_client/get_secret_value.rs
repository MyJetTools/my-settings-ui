use flurl::IntoFlUrl;
use serde::Serialize;

#[derive(Serialize)]
struct RequestModel {
    name: String,
}

pub async fn get_secret_value(secret: String) -> Result<String, String> {
    let result = tokio::spawn(async move {
        let settings_reader = crate::APP_CTX.get_settings_reader().await;

        let url = settings_reader.get_url().await;

        let response = url
            .append_path_segment("api")
            .append_path_segment("secrets")
            .append_path_segment("show")
            .post_json(RequestModel {
                name: secret.clone(),
            })
            .await;

        match response {
            Ok(response) => {
                let response = response.receive_body().await.unwrap();
                Ok(String::from_utf8(response).unwrap())
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
