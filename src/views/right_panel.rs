use dioxus::prelude::*;

use crate::{states::*, views::*};

pub fn right_panel(cx: Scope) -> Element {
    let main_state = use_shared_state::<MainState>(cx).unwrap();

    match &*main_state.read() {
        MainState::Nothing => {
            render!(div {})
        }
        MainState::Templates(_) => {
            render!(templates_list {})
        }
        MainState::Secrets(_) => {
            render!(secrets_list {})
        }

        MainState::Domains(_) => {
            render!(domains_list {})
        }
    }
}
