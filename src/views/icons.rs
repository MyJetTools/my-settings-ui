use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::*;
use dioxus_free_icons::*;

pub fn view_template_icon(cx: Scope) -> Element {
    cx.render(rsx! { Icon { width: 10, height: 10, fill: "white", icon: BsEye } })
}

pub fn edit_icon(cx: Scope) -> Element {
    cx.render(rsx! { Icon { width: 10, height: 10, fill: "white", icon: BsPen } })
}

pub fn delete_icon(cx: Scope) -> Element {
    cx.render(rsx! { Icon { width: 10, height: 10, fill: "white", icon: BsX } })
}

pub fn add_icon(cx: Scope) -> Element {
    cx.render(rsx! { Icon { width: 10, height: 10, fill: "white", icon: BsPlusCircle } })
}

pub fn ok_button_icon(cx: Scope) -> Element {
    cx.render(rsx! { Icon { width: 16, height: 16, fill: "white", icon: BsCheck } })
}

pub fn cancel_button_icon(cx: Scope) -> Element {
    cx.render(rsx! { Icon { width: 16, height: 16, fill: "white", icon: BsX } })
}

pub fn search_icon(cx: Scope) -> Element {
    cx.render(rsx! { Icon { width: 16, height: 16, fill: "gray", icon: BsSearch } })
}

pub fn copy_from_icon(cx: Scope) -> Element {
    cx.render(rsx! { Icon { width: 10, height: 10, fill: "white", icon: BsStickies } })
}

pub fn warning_icon(cx: Scope) -> Element {
    cx.render(rsx! { Icon { width: 16, height: 16, fill: "orange", icon: BsShieldExclamation } })
}

pub fn table_up_icon(cx: Scope) -> Element {
    cx.render(rsx! { Icon { width: 16, height: 16, icon: BsArrowUpShort } })
}
