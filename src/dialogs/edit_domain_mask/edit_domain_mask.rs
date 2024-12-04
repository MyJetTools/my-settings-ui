use dioxus::prelude::*;

use crate::{
    states::*,
    views::{dialog::edit_domain_mask::state::EditDomainMaskState, icons::*},
};

pub fn edit_domain_mask<'s>(mask: String) -> Element {
    let widget_state = use_ref(cx, || EditDomainMaskState::new(cx.props.mask.as_str()));
    let save_button_is_disabled = !widget_state.read().can_be_saved();

    rsx! {
        div { class: "modal-content",
            div { class: "form-floating mb-3",
                input {
                    class: "form-control",
                    value: "{widget_state.read().get_value()}",
                    oninput: move |cx| {
                        let mut widget_state = widget_state.write();
                        widget_state.set_value(cx.value.as_str());
                    }
                }
                label { "Domain mask. Example: *-evn.domain.com" }
            }
        }

        div { class: "modal-footer",
            div { class: "btn-group",
                button {
                    class: "btn btn-primary",
                    disabled: save_button_is_disabled,
                    onclick: move |_| {
                        let main_state = use_shared_state::<MainState>(cx).unwrap().to_owned();
                        let dialog_state = use_shared_state::<DialogState>(cx).unwrap().to_owned();
                        let domain_mask = widget_state.read().get_value().to_string();
                        cx.spawn(async move {
                            save_domain_mask(domain_mask).await.unwrap();
                            main_state.write().set_domains(None);
                            dialog_state.write().hide_dialog();
                        });
                    },
                    ok_button_icon {}
                    "Save"
                }
                button {
                    class: "btn btn-outline-dark",
                    onclick: move |_| {
                        use_shared_state::<DialogState>(cx).unwrap().write().hide_dialog();
                    },
                    cancel_button_icon {}
                    "Cancel"
                }
            }
        }
    }
}

#[server]
async fn save_domain_mask<'s>(domain_mask: String) -> Result<(), ServerFnError> {
    crate::grpc_client::DomainsGrpcClient::save_domain_mask(domain_mask)
        .await
        .unwrap();

    Ok(())
}
