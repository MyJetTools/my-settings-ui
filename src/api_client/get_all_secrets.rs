use flurl::IntoFlUrl;
#[derive(serde::Deserialize)]
pub struct AllSecretsContract {
    pub data: Vec<SecretModel>,
}

#[derive(serde::Deserialize, Clone)]
pub struct SecretModel {
    pub name: String,
    pub created: String,
    pub updated: String,
    #[serde(rename = "templatesAmount")]
    pub templates_amount: usize,
    #[serde(rename = "secretsAmount")]
    pub secrets_amount: usize,
    pub level: usize,
}

pub async fn get_list_of_secrets() -> Result<Vec<SecretModel>, String> {
    let result = tokio::spawn(async move {
        let settings_reader = crate::APP_CTX.get_settings_reader().await;

        let url = settings_reader.get_url().await;

        let response = url
            .append_path_segment("api")
            .append_path_segment("secrets")
            .append_path_segment("getall")
            .post(None)
            .await;

        match response {
            Ok(mut response) => {
                let response: AllSecretsContract = response.get_json().await.unwrap();
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
