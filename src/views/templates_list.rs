use dioxus::prelude::*;
use rust_extensions::date_time::DateTimeAsMicroseconds;

use super::icons::*;
use crate::states::MainState;

pub fn templates_list(cx: Scope) -> Element {
    let main_state = use_shared_state::<MainState>(cx).unwrap();

    match main_state.read().unwrap_as_templates() {
        Some(templates) => {
            let templates = templates.iter().map(|itm| {
                let last_request = if itm.last_request == 0 {
                    "".to_string()
                } else {
                    let last_request = DateTimeAsMicroseconds::new(itm.last_request * 1000);
                    last_request.to_rfc3339()
                };

                rsx! {
                    tr { style: "border-top: 1px solid lightgray",
                        td { "{itm.env}" }
                        td { "{itm.name}" }
                        td { "{itm.created}" }
                        td { "{itm.updated}" }
                        td { "{last_request}" }
                        td {
                            div { class: "btn-group",
                                button { class: "btn btn-sm btn-success", view_template_icon {} }
                                button { class: "btn btn-sm btn-primary", edit_icon {} }
                                button { class: "btn btn-sm btn-danger", delete_icon {} }
                            }
                        }
                    }
                }
            });
            render! {
                table { class: "table table-striped", style: "text-align: left;",
                    tr {
                        th { "Env" }
                        th { "Name" }
                        th { "Created" }
                        th { "Updated" }
                        th { "Last request" }
                        th {
                            div {
                                button { class: "btn btn-sm btn-primary", add_icon {} }
                            }
                        }
                    }

                    templates.into_iter()
                }
            }
        }
        None => {
            load_templates(&cx, &main_state);
            render! { h1 { "loading" } }
        }
    }
}

fn load_templates(cx: &Scope, main_state: &UseSharedState<MainState>) {
    let main_state = main_state.to_owned();

    cx.spawn(async move {
        let response = crate::api_client::get_list_of_templates().await.unwrap();

        main_state.write().set_templates(Some(response));
    });
}
