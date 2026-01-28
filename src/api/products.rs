use dioxus::prelude::*;
#[get("/api/products/list?env_id")]
pub async fn get_list_of_products(env_id: String) -> Result<Vec<String>, ServerFnError> {
    let ctx = crate::server::APP_CTX.get_app_ctx(env_id.as_str()).await;

    let result = ctx.templates_grpc.get_products(()).await.unwrap();

    Ok(result.products)
}
