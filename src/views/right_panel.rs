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
            rsx!(crate::views::templates_page::TemplatesPage {})
        }
        LocationState::Secrets => {
            rsx!(crate::views::secrets_page::SecretsPage {})
        }
    }
}
