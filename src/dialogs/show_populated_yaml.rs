use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_utils::{DataState, RenderState};

use crate::dialogs::*;

#[component]
pub fn ShowPopulatedYaml(
    env_id: Rc<String>,
    product_id: Rc<String>,
    template_id: Rc<String>,
) -> Element {
    let cs = use_signal(|| ShowPopulatedYamlState::new());

    let cs_ra = cs.read();

    let yaml = match get_data(cs, &cs_ra, env_id.clone(), product_id, template_id) {
        Ok(yaml) => yaml,
        Err(err) => return err,
    };

    let content = rsx! {
        textarea { class: "form-control modal-content-full-screen", readonly: true, {yaml} }
    };

    rsx! {
        DialogTemplate { header: "Populated yaml", allocate_max_space: true, content }
    }
}

fn get_data<'s>(
    mut cs: Signal<ShowPopulatedYamlState>,
    cs_ra: &'s ShowPopulatedYamlState,
    env_id: Rc<String>,
    product_id: Rc<String>,
    template_id: Rc<String>,
) -> Result<&'s str, Element> {
    match cs_ra.yaml.as_ref() {
        RenderState::None => {
            let env_id = env_id.to_string();
            let product_id = product_id.to_string();
            let template_id = template_id.to_string();
            spawn(async move {
                match crate::api::templates::load_yaml(env_id, product_id, template_id).await {
                    Ok(result) => {
                        cs.write().yaml.set_loaded(result.yaml);
                    }
                    Err(err) => {
                        cs.write().yaml.set_error(err.to_string());
                    }
                }
            });
            return Err(crate::icons::loading_icon());
        }
        RenderState::Loading => {
            return Err(crate::icons::loading_icon());
        }
        RenderState::Loaded(yaml) => {
            return Ok(yaml.as_str());
        }
        RenderState::Error(err) => {
            return Err(crate::icons::render_error(err));
        }
    }
}

pub struct ShowPopulatedYamlState {
    pub yaml: DataState<String>,
}

impl ShowPopulatedYamlState {
    pub fn new() -> Self {
        Self {
            yaml: DataState::new(),
        }
    }
}
