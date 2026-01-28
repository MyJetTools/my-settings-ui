use dioxus::prelude::*;

use crate::states::MainState;

pub fn select_product(
    main_state: &MainState,
    show_all: Option<&'static str>,
    value: Option<&str>,
    read_only: bool,
    on_change: EventHandler<Option<String>>,
) -> Element {
    let to_render = main_state.products.iter().map(|product_id| {
        rsx! {
            option {
                selected: value == Some(product_id),
                value: product_id.as_str(),
                {product_id.as_str()}
            }
        }
    });

    let all_option = if let Some(show_all) = show_all {
        rsx! {
            option { value: "*", selected: value.is_none(), {show_all} }
        }
    } else {
        rsx! {}
    };

    rsx! {
        select {
            class: "form-select form-select-sm",
            value,
            disabled: read_only,
            onchange: move |ctx| {
                let value = ctx.value();
                if value == "*" {
                    on_change.call(None);
                } else {
                    on_change.call(Some(ctx.value()));
                }

            },
            {all_option}
            {to_render}
        }
    }
}
