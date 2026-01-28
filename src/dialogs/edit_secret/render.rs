use std::rc::Rc;

use dioxus::prelude::*;

use dioxus_utils::*;

use crate::icons::*;
use crate::models::*;
use crate::states::MainState;

use super::super::*;

use super::state::*;

#[component]
pub fn EditSecret(
    env_id: Rc<String>,
    product_id: Option<Rc<String>>,
    secret_id: Rc<String>,
    on_ok: EventHandler<UpdateSecretValueHttpModel>,
) -> Element {
    let mut cs = use_signal(|| EditSecretState::new(secret_id.to_string(), &product_id));
    let cs_ra = cs.read();

    match get_data(cs, &cs_ra, &env_id, &product_id, &secret_id) {
        Ok(_) => {}
        Err(err) => return err,
    };

    let ms = consume_context::<Signal<MainState>>();
    let ms_ra = ms.read();

    let product_select = crate::components::select_product(
        &ms_ra,
        Some("Shared"),
        cs_ra.product_id.as_deref(),
        !cs_ra.new_secret,
        EventHandler::new(move |value| {
            cs.write().product_id = value;
        }),
    );

    let content = rsx! {

        div { class: "form-floating mb-3",
            {product_select}
            label { "Product scope" }
        }

        div { class: "form-floating mb-3",
            input {
                class: "form-control",
                disabled: !cs_ra.new_secret,
                oninput: move |cx| {
                    cs.write().secret_id = cx.value();
                },
                value: cs_ra.secret_id.as_str(),
            }
            label { "Secret name" }
        }

        div { class: "form-floating mb-3",
            input {
                class: "form-control",
                oninput: move |cx| {
                    cs.write().value.value = cx.value();
                },
                value: cs_ra.value.value.as_str(),
            }
            label { "Secret value" }
        }

        div { class: "form-floating mb-3",
            input {
                class: "form-control",
                r#type: "number",
                oninput: move |cx| {
                    cs.write().value.level = cx.value();
                },
                value: cs_ra.value.level.as_str(),
            }
            label { "Secret level" }
        }
    };

    rsx! {

        DialogTemplate {
            header: "Edit secret",
            content,
            ok_button: rsx! {
                button {
                    class: "btn btn-primary",
                    disabled: cs_ra.save_button_is_disabled(),
                    onclick: move |_| {
                        let result = cs.read().get_result();
                        on_ok.call(result);
                        consume_context::<Signal<DialogState>>().set(DialogState::None);
                    },
                    OkButtonIcon {}
                    "Save"
                }
            },
        }
    }
}

fn get_data(
    mut cs: Signal<EditSecretState>,
    cs_ra: &EditSecretState,
    env_id: &str,
    product_id: &Option<Rc<String>>,
    secret_id: &str,
) -> Result<(), Element> {
    match cs_ra.value_on_init.as_ref() {
        RenderState::None => {
            let env_id = env_id.to_string();
            let product_id = match product_id {
                Some(product_id) => Some(product_id.to_string()),
                None => None,
            };
            let secret_id = secret_id.to_string();
            spawn(async move {
                cs.write().value_on_init.set_loading();
                match crate::api::secrets::load_secret_value(env_id, product_id, secret_id).await {
                    Ok(value) => {
                        cs.write().init_value(SecretValue {
                            value: value.value,
                            level: value.level.to_string(),
                        });
                    }
                    Err(err) => {
                        cs.write().value_on_init.set_error(err.to_string());
                    }
                };
            });

            return Err(crate::icons::loading_icon());
        }
        RenderState::Loading => {
            return Err(crate::icons::loading_icon());
        }

        RenderState::Loaded(_) => {
            return Ok(());
        }

        RenderState::Error(err) => {
            return Err(crate::icons::render_error(err));
        }
    }
}
