use dioxus::prelude::*;

pub fn render_bool_checkbox(enabled: bool, onclick: EventHandler<bool>) -> Element {
    if enabled {
        rsx! {
            img {
                style: "cursor: pointer;width: 20px; height: 20px;",
                src: "/assets/img/enabled.webp",
                onclick: move |_| { onclick.call(false) },

            }
        }
    } else {
        rsx! {
            img {
                style: "cursor: pointer;width: 20px; height: 20px;",
                src: "/assets/img/unchecked.webp",
                onclick: move |_| { onclick.call(true) },
            }
        }
    }
}
