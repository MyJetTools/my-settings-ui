use dioxus::prelude::*;

use crate::dialogs::*;

#[component]
pub fn RenderDialog() -> Element {
    let dialog_state = {
        let dialog_state = consume_context::<Signal<DialogState>>();
        let read_access = dialog_state.read();
        read_access.clone()
    };

    match dialog_state {
        DialogState::None => None,
        DialogState::Confirmation { content, on_ok } => {
            return rsx! {
                ConfirmationDialog { content, on_ok }
            }
        }

        DialogState::EditSecret { name, on_ok } => {
            return rsx! {
                EditSecret { name, on_ok }
            }
        }
        DialogState::SecretUsage(secret) => {
            rsx! {
                ShowSecretUsageByTemplate { secret }
            }
        }

        DialogState::SecretUsageBySecret(secret) => {
            rsx! {
                ShowSecretUsageBySecret { secret }
            }
        }

        DialogState::ShowSecret(secret) => {
            rsx! {
                ShowSecret { secret }
            }
        }

        DialogState::EditTemplate {
            env,
            name,
            init_from_other_template,
        } => {
            rsx! {
                EditTemplate { env, name, init_from_other_template }
            }
        }

        DialogState::ShowPopulatedYaml { env, name } => {
            rsx! {
                ShowPopulatedYaml { env, name }
            }
        }
    }
}
