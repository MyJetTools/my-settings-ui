use dioxus::prelude::*;

pub fn view_template_icon() -> Element {
    rsx! {
        img { class: "btn-icon-sm", src: "/assets/img/ico-view.svg" }
    }
}
#[component]
pub fn EditIcon() -> Element {
    rsx! {
        img { class: "btn-icon-sm", src: "/assets/img/ico-edit.svg" }
    }
}
#[component]
pub fn DeleteIcon() -> Element {
    rsx! {
        img { class: "btn-icon-sm", src: "/assets/img/ico-delete.svg" }
    }
}
#[component]
pub fn AddIcon() -> Element {
    rsx! {
        img { class: "btn-icon-sm", src: "/assets/img/ico-add.svg" }
    }
}
#[component]
pub fn OkButtonIcon() -> Element {
    rsx! {
        img { class: "btn-icon-sm", src: "/assets/img/ico-ok.svg" }
    }
}

#[component]
pub fn CancelButtonIcon() -> Element {
    rsx! {
        img { class: "btn-icon-sm", src: "/assets/img/ico-cancel.svg" }
    }
}
#[component]
pub fn SearchIcon() -> Element {
    rsx! {
        img { class: "btn-icon-sm", src: "/assets/img/ico-search.svg" }
    }
}

#[component]
pub fn CopyFromIcon() -> Element {
    rsx! {
        img { class: "btn-icon-sm", src: "/assets/img/ico-copy.svg" }
    }
}
#[component]
pub fn WarningIcon() -> Element {
    rsx! {
        img { class: "btn-icon-sm", src: "/assets/img/ico-warning.svg" }
    }
}
#[component]
pub fn TableUpIcon() -> Element {
    rsx! {
        img { class: "btn-icon-sm", src: "/assets/img/ico-up.svg" }
    }
}

#[component]
pub fn LoadingIcon() -> Element {
    rsx! {
        span { class: "loader" }
    }
}
