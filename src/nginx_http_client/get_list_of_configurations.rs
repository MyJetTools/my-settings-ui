use serde::*;

pub async fn get_list_of_configurations() -> Vec<NginxSetupHttpModel> {
    let nginx_api_url = crate::APP_CTX.settings.get_nginx_api();

    let mut response = flurl::FlUrl::new(nginx_api_url)
        .append_path_segment("api")
        .append_path_segment("nginx")
        .append_path_segment("http")
        .append_path_segment("v1")
        .append_path_segment("all")
        .get()
        .await
        .unwrap();

    let result = response.get_json().await.unwrap();

    result
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NginxSetupHttpModel {
    pub domain: String,
    pub port: u16,
    pub protocol: String,
    #[serde(rename = "sslCertificate")]
    pub ssl_certificate: Option<String>,
    #[serde(rename = "clientCertCaCn")]
    pub client_cert_ca_cn: Option<String>,
    pub templates: Option<Vec<String>>,
    pub locations: Vec<NginxLocationSetupHttpModel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NginxLocationSetupHttpModel {
    pub location: String,
    #[serde(rename = "proxyPass")]
    pub proxy_pass: String,
    pub templates: Option<Vec<String>>,
}
