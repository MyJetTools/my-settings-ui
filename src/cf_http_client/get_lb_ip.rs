pub async fn get_lb_ip() -> String {
    let cloud_flare_url = crate::APP_CTX.settings.get_cloud_flare_url();

    let response = flurl::FlUrl::new(cloud_flare_url)
        .append_path_segment("api")
        .append_path_segment("InternetIp")
        .get()
        .await
        .unwrap();

    let ip = response.receive_body().await.unwrap();

    String::from_utf8(ip).unwrap()
}
