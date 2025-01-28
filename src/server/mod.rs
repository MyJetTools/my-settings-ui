pub mod app_ctx;
pub mod grpc_client;
pub mod settings;

//mod nginx_http_client;
use app_ctx::AppCtx;

lazy_static::lazy_static! {
    pub static ref APP_CTX: AppCtx = {
        AppCtx::new()
    };
}

pub mod templates_grpc {
    tonic::include_proto!("templates");
}

pub mod secrets_grpc {
    tonic::include_proto!("secrets");
}

//pub mod domains_grpc {
//    tonic::include_proto!("domains");
//}
