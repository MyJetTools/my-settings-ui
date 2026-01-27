use std::rc::Rc;

use dioxus::prelude::*;

use dioxus_utils::{console_log, DataState, RenderState};

use crate::models::*;

use super::*;

#[component]
pub fn ShowSecretUsageByTemplate(
    env_id: Rc<String>,
    product_id: Option<Rc<String>>,
    secret_id: Rc<String>,
) -> Element {
    console_log(format!("Secret Usage: {}", secret_id.as_str()).as_str());

    let cs = use_signal(|| ShowSecretUsageByTemplateState::new());

    let cs_ra = cs.read();

    let data = match get_data(
        cs,
        &cs_ra,
        env_id.clone(),
        product_id.clone(),
        secret_id.clone(),
    ) {
        Ok(data) => data,
        Err(err) => return err,
    };

    let content = data.into_iter().map(|itm| {
        let items = itm.yaml.split("\n").map(|itm| {
            if itm.contains(secret_id.as_str()) {
                rsx! {
                    div { style: "color:black;", {itm} }
                }
            } else {
                rsx! {
                    div { style: "color:lightgray", {itm} }
                }
            }
        });

        rsx! {
            h4 { "{itm.product_id}/{itm.template_id}" }
            {items}
            hr {}
        }
    });

    rsx! {
        DialogTemplate {
            header: "Usage of secret {secret_id.as_str()}",
            width: "95%",
            content: rsx! {
                div { style: "text-align:left", class: "dialog-max-content", {content} }
            },
        }
    }
}

fn get_data<'s>(
    mut cs: Signal<ShowSecretUsageByTemplateState>,
    cs_ra: &'s ShowSecretUsageByTemplateState,
    env_id: Rc<String>,
    product_id: Option<Rc<String>>,
    secret_id: Rc<String>,
) -> Result<&'s [TemplateUsageApiModel], Element> {
    match cs_ra.data.as_ref() {
        RenderState::None => {
            let env_id = env_id.to_string();
            let secret_id = secret_id.to_string();
            let product_id = product_id.map(|itm| itm.to_string());
            spawn(async move {
                match crate::api::secrets::load_secret_usage_by_templates(
                    env_id, product_id, secret_id,
                )
                .await
                {
                    Ok(result) => {
                        cs.write().data.set_loaded(result);
                    }
                    Err(err) => {
                        cs.write().data.set_error(err.to_string());
                    }
                }
            });
            return Err(crate::icons::loading_icon());
        }
        RenderState::Loading => {
            return Err(crate::icons::loading_icon());
        }
        RenderState::Loaded(data) => return Ok(data.as_slice()),
        RenderState::Error(err) => {
            return Err(crate::icons::render_error(err));
        }
    }
}

pub struct ShowSecretUsageByTemplateState {
    data: DataState<Vec<TemplateUsageApiModel>>,
}

impl ShowSecretUsageByTemplateState {
    pub fn new() -> Self {
        Self {
            data: DataState::new(),
        }
    }
}
