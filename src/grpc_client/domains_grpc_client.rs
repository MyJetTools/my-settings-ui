use crate::APP_CTX;
use my_grpc_extensions::client::*;
#[generate_grpc_client(
    proto_file: "./proto/DomainsService.proto",
    crate_ns: "crate::domains_grpc",
    retries: 3,
    request_timeout_sec: 1,
    ping_timeout_sec: 1,
    ping_interval_sec: 3,
)]
pub struct DomainsGrpcClient;

impl DomainsGrpcClient {
    pub async fn get() -> Result<DomainsInfoGrpcResponse, String> {
        let result = APP_CTX.domains_grpc_client.get_domains_info(()).await;

        match result {
            Ok(result) => Ok(result),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    pub async fn save_domain_mask(domain_mask: String) -> Result<(), String> {
        let result = APP_CTX
            .domains_grpc_client
            .set_domain_mask(SetDomainMaskRequest { domain_mask })
            .await;

        match result {
            Ok(result) => Ok(result),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    pub async fn save(
        product_name: String,
        is_cloud_flare_proxy: bool,
        internal_domain_name: String,
    ) -> Result<(), String> {
        let result = APP_CTX
            .domains_grpc_client
            .set_product_info(DomainProductGrpcInfo {
                product_name,
                is_cloud_flare_proxy,
                internal_domain_name,
            })
            .await;

        match result {
            Ok(result) => Ok(result),
            Err(err) => Err(format!("{:?}", err)),
        }
    }
}
