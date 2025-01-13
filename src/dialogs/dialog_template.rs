use dioxus::prelude::*;
use rust_extensions::StrOrString;

use super::*;

#[component]
pub fn DialogTemplate(
    header: String,
    header_content: Option<Element>,
    content: Element,
    allocate_max_space: Option<bool>,
    ok_button: Option<Element>,
    width: Option<String>,
) -> Element {
    let allocate_max_space = allocate_max_space.unwrap_or_default();
    let (id, content_id) = if allocate_max_space {
        ("dialog-window-max", "dialog-max-content")
    } else {
        ("dialog-window", "")
    };

    let width_style: StrOrString = if let Some(width) = width.as_ref() {
        format!("width:{}", width).into()
    } else {
        "".into()
    };
    let buttons = if allocate_max_space {
        rsx! {
            div {}
        }
    } else {
        match ok_button {
            Some(ok_button) => {
                rsx! {
                    div { class: "btn-group",
                        {ok_button}
                        button {
                            class: "btn btn-outline-primary",
                            onclick: move |_| {
                                consume_context::<Signal<DialogState>>().set(DialogState::None);
                            },
                            "Cancel"
                        }
                    }
                }
            }
            None => {
                rsx! {
                    button {
                        class: "btn btn-outline-primary",
                        onclick: move |_| {
                            consume_context::<Signal<DialogState>>().set(DialogState::None);
                        },
                        "Close"
                    }
                }
            }
        }
    };

    let separator = if allocate_max_space {
        rsx! {}
    } else {
        rsx! {
            hr {}
        }
    };

    rsx! {
        div { id: "dialog-background",
            div { style: "{width_style.as_str()}", id,
                div { id: "dialog-header",
                    table { style: "width:100%",
                        tr {
                            td {

                                h2 { {header} }
                            }
                            td { {header_content} }
                            td { style: "vertical-align:top;text-align:right;cursor:pointer",
                                div {
                                    onclick: move |_| {
                                        consume_context::<Signal<DialogState>>().set(DialogState::None);
                                    },
                                    "X"
                                }
                            }
                        }
                    }
                }

                div { id: content_id, {content} }

                {separator}

                div { style: "text-align:right", {buttons} }
            }
        }
    }
}
