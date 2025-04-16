use crate::message_form::send_message_form;
use crate::message_table::message_table;
use axum::{extract::State, response::Html};
use maud::html;
use shared::AppState;

pub async fn index(State(state): State<AppState>) -> Html<String> {
    state.metrics.api_calls_total.inc();
    let messages = state.repo.get_all_messages().await.unwrap_or_default();

    let page = html! {
        (maud::DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                title { "Send Message" }
                script src="https://unpkg.com/htmx.org@1.9.2" {}
            }
            body {
                div id="parent-div" {
                    h3 { "All messages:" }
                    (message_table(&messages))
                    h3 { "Send a Message" }
                    (send_message_form())
                }
            }
        }
    };

    Html(page.into_string())
}
