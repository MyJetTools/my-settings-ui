pub async fn get_dns_records(domain: String) -> Vec<super::CfDnsRecordRestApiModel> {
    let cloud_flare_bridge_url = crate::APP_CTX.settings.get_cloud_flare_url();

    let mut response = flurl::FlUrl::new(cloud_flare_bridge_url)
        .append_path_segment("api")
        .append_path_segment("DnsZone")
        .append_query_param("domain", Some(domain))
        .get()
        .await
        .unwrap();

    response.get_json().await.unwrap()
}
