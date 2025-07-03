use dioxus::prelude::*;

pub fn render_bool_checkbox(enabled: bool, onclick: EventHandler<bool>) -> Element {
    if enabled {
        rsx! {
            img {
                style: "cursor: pointer",
                src: "/assets/img/enabled.webp",
                onclick: move |_| { onclick.call(false) },
                style: "width: 20px; height: 20px;",
            }
        }
    } else {
        rsx! {
            img {
                style: "cursor: pointer",
                src: "/assets/img/unchecked.webp",
                onclick: move |_| { onclick.call(false) },
                style: "width: 20px; height: 20px;",
            }
        }
    }
}
