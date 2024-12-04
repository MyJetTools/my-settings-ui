use crate::views::NginxConfigHttpModel;

use super::{NginxLocationSetupHttpModel, NginxSetupHttpModel};

pub async fn save_nginx_configuration(domain: String, model: NginxConfigHttpModel) {
    let nginx_api_url = crate::APP_CTX.settings.get_nginx_api();

    let model = NginxSetupHttpModel {
        domain,
        port: 443,
        protocol: "Https2".to_string(),
        ssl_certificate: None,
        client_cert_ca_cn: model.ca,
        templates: if let Some(templates) = model.template {
            Some(vec![templates])
        } else {
            None
        },
        locations: model
            .routes
            .into_iter()
            .map(|itm| {
                let proxy_pass = itm.1.proxy_to;
                let templates = if let Some(template) = itm.1.template {
                    Some(vec![template])
                } else {
                    None
                };

                NginxLocationSetupHttpModel {
                    location: itm.0,
                    proxy_pass,
                    templates,
                }
            })
            .collect::<Vec<_>>(),
    };

    let result = flurl::FlUrl::new(nginx_api_url)
        .append_path_segment("api")
        .append_path_segment("nginx")
        .append_path_segment("http")
        .append_path_segment("v1")
        .post_json(model)
        .await
        .unwrap();

    println!("Update Nginx Status code: {:?}", result.get_status_code());
}
