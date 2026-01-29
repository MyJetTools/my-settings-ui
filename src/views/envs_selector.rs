use dioxus::prelude::*;

use crate::states::*;

#[component]
pub fn EnvsSelector() -> Element {
    let main_state = consume_context::<Signal<MainState>>();

    let selected_env = crate::storage::selected_env::get();

    let main_state_read_access = main_state.read();

    let envs_options = if let Some(envs) = main_state_read_access.envs.try_unwrap_as_loaded() {
        envs.clone().into_iter().map(|env| {
            if selected_env.as_str() == env.as_str() {
                rsx! {
                    option { selected: true, {env.as_str()} }
                }
            } else {
                rsx! {
                    option { {env.as_str()} }
                }
            }
        })
    } else {
        return rsx! {
            option {}
        };
    };

    rsx! {
        select {
            class: "form-select",
            style: "background-color: white;",

            value: selected_env.as_str(),

            oninput: move |ctx| {
                let value = ctx.value();
                crate::storage::selected_env::save(value.as_str());
                consume_context::<Signal<MainState>>()
                    .write()
                    .drop_data();
            },
            {envs_options}
        }
    }
}
