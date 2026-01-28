use crate::models::*;
use dioxus::prelude::*;

#[get("/api/envs", headers: dioxus::fullstack::HeaderMap)]
pub async fn get_envs() -> Result<EnvsHttpResponse, ServerFnError> {
    let user_id = {
        if let Some(user) = headers.get("x-ssl-user") {
            user.to_str().unwrap().to_string()
        } else {
            "".to_string()
        }
    };

    println!("Sending envs for user: [{}]", user_id);

    let (envs, prompt_ssh_pass_key) = crate::server::APP_CTX.get_envs(&user_id).await;

    Ok(EnvsHttpResponse {
        name: user_id,
        envs,
        prompt_ssh_pass_key,
    })
}
