use my_grpc_extensions::client::*;
#[generate_grpc_client(
    proto_file: "./proto/SecretsService.proto",
    crate_ns: "crate::server::secrets_grpc",
    retries: 3,
    request_timeout_sec: 1,
    ping_timeout_sec: 1,
    ping_interval_sec: 3,
)]
pub struct SecretsGrpcClient;
