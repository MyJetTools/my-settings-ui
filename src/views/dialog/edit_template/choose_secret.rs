use dioxus::prelude::*;

use crate::{secrets_grpc::SecretListItem, views::dialog::select_secret};

#[derive(Props)]
pub struct ChooseSecretProps<'s> {
    pub on_selected: EventHandler<'s, String>,
}

pub fn choose_secret<'s>(cx: Scope<'s, ChooseSecretProps<'s>>) -> Element {
    let filter = use_state(cx, || "".to_string());
    let secret_name: &UseState<String> = use_state(cx, || "".to_string());

    let secrets: &UseState<Option<Vec<SecretListItem>>> = use_state(cx, || None);

    let mode = use_state(cx, || ChooseSecretMode::Select);

    let secret_value = use_state(cx, || "".to_string());

    let secret_level = use_state(cx, || "".to_string());

    let content = match mode.get() {
        ChooseSecretMode::Select => {
            if secrets.get().is_none() {
                load_secrets(&cx, secrets);
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
                            add_secret(
                                cx,
                                mode,
                                secrets,
                                secret_name.get(),
                                secret_value.get(),
                                secret_level.get().parse::<i32>().unwrap(),
                            );
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
                        copy_secret_value(cx, &value, secret_value);
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

fn load_secrets<'s>(
    cx: &Scope<'s, ChooseSecretProps<'s>>,
    secrets: &UseState<Option<Vec<SecretListItem>>>,
) {
    let secrets = secrets.to_owned();
    cx.spawn(async move {
        let secrets_values = crate::grpc_client::SecretsGrpcClient::get_all_secrets()
            .await
            .unwrap();

        secrets.set(Some(secrets_values));
    });
}

fn add_secret<'s>(
    cx: &Scoped<'s, ChooseSecretProps<'s>>,
    mode: &UseState<ChooseSecretMode>,
    secrets: &UseState<Option<Vec<SecretListItem>>>,
    name: &str,
    value: &str,
    level: i32,
) {
    let name = name.to_string();
    let value = value.to_string();

    let mode = mode.to_owned();

    let secrets = secrets.to_owned();

    cx.spawn(async move {
        crate::grpc_client::SecretsGrpcClient::save_secret(name.clone(), value, level)
            .await
            .unwrap();

        mode.set(ChooseSecretMode::Select);
        secrets.set(None);
    })
}

fn copy_secret_value<'s>(
    cx: &Scoped<'s, ChooseSecretProps<'s>>,
    secret_name: &str,
    secret_value: &UseState<String>,
) {
    let secret_name = secret_name.to_string();
    let secret_value = secret_value.to_owned();

    cx.spawn(async move {
        let secret_model = crate::grpc_client::SecretsGrpcClient::get_secret(secret_name.clone())
            .await
            .unwrap();

        if secret_model.name.len() > 0 {
            secret_value.set(secret_model.value);
        }
    });
}

pub enum ChooseSecretMode {
    Select,
    Add,
}

fn has_secret(secrets: &Option<Vec<SecretListItem>>, secret_to_find: &str) -> bool {
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
