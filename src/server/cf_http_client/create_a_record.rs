use serde::*;

pub async fn create_a_record(domain: String, proxied: bool, ip: String) {
    let cloud_flare_bridge_url = crate::APP_CTX.settings.get_cloud_flare_url();
    flurl::FlUrl::new(cloud_flare_bridge_url)
        .append_path_segment("api")
        .append_path_segment("DnsZone")
        .append_path_segment("ARecord")
        .post_json(CreateARecordRequest {
            domain,
            proxied,
            ip,
        })
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateARecordRequest {
    pub domain: String,
    pub proxied: bool,
    pub ip: String,
}
