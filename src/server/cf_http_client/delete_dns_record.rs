pub async fn delete_dns_record(domain_zone: &str, id: &str) {
    let cloud_flare_bridge_url = crate::APP_CTX.settings.get_cloud_flare_url();
    flurl::FlUrl::new(cloud_flare_bridge_url.as_str())
        .append_path_segment("api")
        .append_path_segment("DnsZone")
        .append_path_segment("ARecord")
        .append_query_param("domain", Some(domain_zone))
        .append_query_param("id", Some(id))
        .delete()
        .await
        .unwrap();
}
