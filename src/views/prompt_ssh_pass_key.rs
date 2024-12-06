use dioxus::prelude::*;

use crate::MainState;
#[component]
pub fn PromptSshPassKey() -> Element {
    let mut component_state = use_signal(|| PromptSshPassKeyState::new());
    let component_state_read_access = component_state.read();
    rsx! {
        div { style: "width: 600px;margin: auto;",
            h1 { "Enter SSH Pass Key" }
            div { "Enter the SSH Pass Key to use for the SSH connection" }
            div { class: "form-floating mb-3",
                input {
                    r#type: "password",
                    class: "form-control",
                    id: "floatingInput",
                    placeholder: "SSH Pass Key",
                    oninput: move |cx| {
                        component_state.write().pass_phrase = cx.value();
                    },
                    value: component_state_read_access.pass_phrase.as_str()
                }
            }
            div { class: "d-grid gap-2",
                button {
                    class: "btn btn-primary",
                    onclick: move |_| {
                        let pass_phrase = component_state.read().pass_phrase.clone();
                        spawn(async move {
                            apply_pass_phrase(pass_phrase).await.unwrap();
                            consume_context::<Signal<MainState>>().write().prompt_ssh_key = Some(false);
                        });
                    },
                    "Submit"
                }
            }
        }
    }
}

pub struct PromptSshPassKeyState {
    pub pass_phrase: String,
}
impl PromptSshPassKeyState {
    pub fn new() -> Self {
        Self {
            pass_phrase: String::new(),
        }
    }
}

#[server]
async fn apply_pass_phrase(pass_phrase: String) -> Result<(), ServerFnError> {
    crate::server::APP_CTX
        .private_key_resolver
        .set_pass_phrase(pass_phrase)
        .await;

    Ok(())
}
