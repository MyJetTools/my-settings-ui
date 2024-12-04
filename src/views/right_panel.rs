use dioxus::prelude::*;

use crate::{states::*, views::*};

#[component]
pub fn RightPanel() -> Element {
    let main_state = consume_context::<Signal<MainState>>();

    let main_state_read_access = main_state.read();

    match *main_state_read_access {
        MainState::Nothing => {
            rsx!(
                div {}
            )
        }
        MainState::Templates(_) => {
            rsx!(
                TemplatesList {}
            )
        }
        MainState::Secrets(_) => {
            rsx!(
                SecretsList {}
            )
        }
    }
}
