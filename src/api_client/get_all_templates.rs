use flurl::IntoFlUrl;
#[derive(serde::Deserialize)]
pub struct AllTemplatesContract {
    pub data: Vec<TemplateModel>,
}

#[derive(serde::Deserialize, Clone)]
pub struct TemplateModel {
    pub env: String,
    pub name: String,
    pub created: String,
    pub updated: String,
    #[serde(rename = "lastRequest")]
    pub last_request: i64,
}

pub async fn get_list_of_templates() -> Result<Vec<TemplateModel>, String> {
    let result = tokio::spawn(async move {
        let settings_reader = crate::APP_CTX.get_settings_reader().await;

        let url = settings_reader.get_url().await;

        let response = url
            .append_path_segment("api")
            .append_path_segment("templates")
            .append_path_segment("getall")
            .post(None)
            .await;

        match response {
            Ok(mut response) => {
                let response: AllTemplatesContract = response.get_json().await.unwrap();
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
