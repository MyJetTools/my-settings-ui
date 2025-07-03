use dioxus::prelude::*;

pub fn render_bool_checkbox(enabled: bool) -> Element {
    if enabled {
        rsx! {
            img {
                src: "/assets/img/enabled.webp",
                style: "width: 20px; height: 20px;",
            }
        }
    } else {
        rsx! {
            img {
                src: "/assets/img/unchecked.webp",
                style: "width: 20px; height: 20px;",
            }
        }
    }
}
