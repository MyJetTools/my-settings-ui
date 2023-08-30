use flurl::IntoFlUrl;
use serde::Serialize;

#[derive(Serialize)]
struct RequestModel {
    env: String,
    name: String,
}

pub async fn get_template_value(env: String, name: String) -> Result<String, String> {
    let result = tokio::spawn(async move {
        let settings_reader = crate::APP_CTX.get_settings_reader().await;

        let url = settings_reader.get_url().await;

        let response = url
            .append_path_segment("api")
            .append_path_segment("templates")
            .append_path_segment("get")
            .post_json(RequestModel { env, name })
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
