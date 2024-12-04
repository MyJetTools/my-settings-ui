use dioxus::prelude::*;

use crate::{states::*, views::*};

#[component]
pub fn RightPanel() -> Element {
    let main_state = consume_context::<Signal<MainState>>();

    let main_state_read_access = main_state.read();

    match main_state_read_access.location {
        LocationState::None => {
            rsx!(div {})
        }
        LocationState::Templates => {
            rsx!(TemplatesList {})
        }
        LocationState::Secrets => {
            rsx!(SecretsList {})
        }
    }
}
