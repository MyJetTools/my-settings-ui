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
            product_id,
            secret_id,
            on_ok,
        } => {
            return rsx! {
                EditSecret {
                    env_id,
                    product_id,
                    secret_id,
                    on_ok,
                }
            }
        }
        DialogState::SecretUsage {
            env_id,
            product_id,
            secret_id,
        } => {
            rsx! {
                ShowSecretUsageByTemplate { env_id, product_id, secret_id }
            }
        }

        DialogState::SecretUsageBySecret { env_id, secret_id } => {
            rsx! {
                ShowSecretUsageBySecret { env_id, secret_id }
            }
        }

        DialogState::ShowSecret {
            env_id,
            product_id,
            secret_id,
        } => {
            rsx! {
                ShowSecret { env_id, product_id, secret_id }
            }
        }

        DialogState::EditTemplate {
            env_id,
            data,
            on_ok,
        } => {
            rsx! {
                EditTemplate { env_id, data, on_ok }
            }
        }

        DialogState::ShowPopulatedYaml {
            env_id,
            product_id,
            template_id,
        } => {
            rsx! {
                ShowPopulatedYaml { env_id, product_id, template_id }
            }
        }

        DialogState::SnapshotToExport(yaml) => {
            rsx! {
                ShowTemplateToExport { yaml }
            }
        }

        DialogState::SnapshotToImport(on_ok) => {
            rsx! {
                SnapshotToImport { on_ok }
            }
        }

        DialogState::CopyToEnvConfirmation { from_env_id, on_ok } => {
            rsx! {
                CopyToEnvDialog { from_env_id, on_ok }
            }
        }
    }
}
