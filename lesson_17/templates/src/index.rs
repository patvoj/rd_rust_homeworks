use crate::message_form::{messages_table, send_message_form};
use axum::response::Html;
use maud::html;

pub async fn index() -> Html<String> {
    let page = html! {
        (maud::DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                title { "Send Message" }
                script src="https://unpkg.com/htmx.org@1.9.2" {}
            }
            body {
                h3 { "All messages:" }
                (messages_table())
                h3 { "Send a Message" }
                (send_message_form())
            }
        }
    };
    Html(page.into_string())
}
