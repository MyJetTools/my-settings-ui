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
        DialogState::None => rsx! {},
        DialogState::Confirmation { content, on_ok } => {
            return rsx! {
                ConfirmationDialog { content, on_ok }
            }
        }

        DialogState::EditSecret {
            env_id,
            name,
            on_ok,
        } => {
            return rsx! {
                EditSecret { env_id, name, on_ok }
            }
        }
        DialogState::SecretUsage { env_id, secret } => {
            rsx! {
                ShowSecretUsageByTemplate { env_id, secret }
            }
        }

        DialogState::SecretUsageBySecret { env_id, secret } => {
            rsx! {
                ShowSecretUsageBySecret { env_id, secret }
            }
        }

        DialogState::ShowSecret { env_id, secret } => {
            rsx! {
                ShowSecret { env_id, secret }
            }
        }

        DialogState::EditTemplate {
            env_id,
            env,
            name,
            init_from_other_template,
            on_ok,
        } => {
            rsx! {
                EditTemplate {
                    env_id,
                    env,
                    name,
                    init_from_other_template,
                    on_ok,
                }
            }
        }

        DialogState::ShowPopulatedYaml { env_id, env, name } => {
            rsx! {
                ShowPopulatedYaml { env_id, env, name }
            }
        }
    }
}
