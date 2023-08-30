use dioxus::prelude::*;

#[derive(Props, PartialEq, Eq)]
pub struct ShowPopulatedYamlProps {
    pub env: String,
    pub name: String,
}

pub fn show_populated_yaml<'s>(cx: Scope<'s, ShowPopulatedYamlProps>) -> Element {
    let yaml = use_state(cx, || "".to_string());

    if yaml.is_empty() {
        load_yaml(&cx, yaml);

        return render! {
            div { class: "modal-content", div { class: "form-control modal-content-full-screen", "Loading..." } }
        };
    }

    render! {
        div { class: "modal-content",
            textarea { class: "form-control modal-content-full-screen", readonly: true, yaml.as_str() }
        }
    }
}

fn load_yaml<'s>(cx: &'s Scope<'s, ShowPopulatedYamlProps>, state: &UseState<String>) {
    let env = cx.props.env.clone();
    let name = cx.props.name.clone();

    let state = state.to_owned();

    cx.spawn(async move {
        let yaml = crate::grpc_client::TemplatesGrpcClient::get_populated_template(env, name)
            .await
            .unwrap();

        state.set(yaml);
    });
}
