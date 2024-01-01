use dioxus::prelude::*;

use crate::views::{
    dialog::{load_secret, save_secret, select_secret},
    load_secrets, SecretListItemApiModel,
};

#[derive(Props)]
pub struct ChooseSecretProps<'s> {
    pub on_selected: EventHandler<'s, String>,
}

pub fn choose_secret<'s>(cx: Scope<'s, ChooseSecretProps<'s>>) -> Element {
    let filter = use_state(cx, || "".to_string());
    let secret_name: &UseState<String> = use_state(cx, || "".to_string());

    let secrets: &UseState<Option<Vec<SecretListItemApiModel>>> = use_state(cx, || None);

    let mode = use_state(cx, || ChooseSecretMode::Select);

    let secret_value = use_state(cx, || "".to_string());

    let secret_level = use_state(cx, || "".to_string());

    let content = match mode.get() {
        ChooseSecretMode::Select => {
            if secrets.get().is_none() {
                let secrets = secrets.to_owned();
                cx.spawn(async move {
                    let result = load_secrets().await.unwrap();
                    secrets.set(Some(result));
                })
            }

            match secrets.get() {
                Some(values) => {
                    if filter.get().len() < 3 {
                        vec![]
                    } else {
                        let mut result = Vec::new();

                        for value in values {
                            if value.name.to_lowercase().contains(filter.get().as_str()) {
                                result.push(rsx! {
                                    div {
                                        button {
                                            class: "btn btn-sm btn-primary",
                                            onclick: move |_| {
                                                cx.props.on_selected.call(value.name.to_string());
                                            },
                                            "Copy"
                                        }
                                        "{value.name}"
                                    }
                                })
                            }
                        }

                        result
                    }
                }
                None => vec![],
            }
        }
        ChooseSecretMode::Add => {
            let btn = if has_secret(&secrets.get(), secret_name.get()) {
                rsx! { div { class: "alert alert-danger", "Secret already exists" } }
            } else {
                rsx! {
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            let name = secret_name.get().to_string();
                            let value = secret_value.get().to_string();
                            let level = secret_level.get().parse::<i32>().unwrap();
                            cx.spawn(async move {
                                save_secret(name, value, level).await.unwrap();
                            });
                        },
                        "Add new secret"
                    }
                }
            };
            let result = rsx! {
                div { style: "margin-top:10px", class: "form-floating mb-3",
                    input {
                        class: "form-control",
                        value: "{secret_value.get()}",
                        oninput: move |cx| {
                            secret_value.set(cx.value.to_string());
                        }
                    }
                    label { "Secret value" }
                }

                div { class: "form-floating mb-3",
                    input {
                        class: "form-control",
                        r#type: "number",
                        value: "{secret_level.get()}",
                        oninput: move |cx| {
                            secret_level.set(cx.value.to_string());
                        }
                    }
                    label { "Secret level" }
                }

                btn,

                hr {}
                h4 { "Copy value from other secret" }
                select_secret {
                    on_selected: move |value: String| {
                        let secret_value = secret_value.to_owned();
                        cx.spawn(async move {
                            let result = load_secret(value).await.unwrap();
                            secret_value.set(result.value);
                        });
                    }
                }
            };

            vec![result]
        }
    };

    let btn = match mode.get() {
        ChooseSecretMode::Select => {
            rsx! {
                button {
                    class: "btn btn-outline-secondary",
                    style: "width:150px",
                    onclick: move |_| {
                        mode.set(ChooseSecretMode::Add);
                    },
                    "Select secret"
                }
            }
        }
        ChooseSecretMode::Add => {
            rsx! {
                button {
                    class: "btn btn-outline-secondary",
                    style: "width:150px",
                    onclick: move |_| {
                        mode.set(ChooseSecretMode::Select);
                    },
                    "Add secret"
                }
            }
        }
    };

    let text = match mode.get() {
        ChooseSecretMode::Select => "Search secret",
        ChooseSecretMode::Add => "Add secret",
    };

    render! {
        div { style: "margin-top:5px; width:100%", class: "input-group",
            input {
                class: "form-control",
                placeholder: text,
                oninput: move |cx| {
                    filter.set(cx.value.to_lowercase());
                    secret_name.set(cx.value.to_string());
                }
            }
            btn
        }
        div { style: "height:70vh; overflow-y: auto;", content.into_iter() }
    }
}

pub enum ChooseSecretMode {
    Select,
    Add,
}

fn has_secret(secrets: &Option<Vec<SecretListItemApiModel>>, secret_to_find: &str) -> bool {
    match secrets {
        Some(values) => {
            for value in values {
                if value.name == secret_to_find {
                    return true;
                }
            }
        }
        None => {}
    }

    false
}
