use dioxus::prelude::*;
use rust_extensions::StrOrString;

use super::*;

#[component]
pub fn DialogTemplate(
    header: String,
    header_content: Option<VNode>,
    content: Option<VNode>,
    allocate_max_space: Option<bool>,
    ok_button: Option<VNode>,
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
        if ok_button.is_none() {
            rsx! {
                button {
                    class: "btn btn-outline-primary",
                    onclick: move |_| {
                        consume_context::<Signal<DialogState>>().set(DialogState::None);
                    },
                    "Close"
                }
            }
        } else {
            rsx! {
                div { class: "btn-group",
                    {ok_button},
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
    };

    let separator = if allocate_max_space {
        None
    } else {
        rsx! {
            hr {}
        }
    };

    rsx! {
        div { id: "dialog-background",
            div { style: "{width_style}", id,
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

                {separator},

                div { style: "text-align:right", {buttons} }
            }
        }
    }
}
