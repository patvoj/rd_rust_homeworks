use maud::{html, Markup};

pub fn send_message_form() -> Markup {
    html! {
        form hx-post="/messages" hx-target="#parent-div" hx-swap="innerHTML" method="post" {
            input type="text" name="content" placeholder="Type your message..." required;
            button type="submit" { "Send" }
        }
    }
}

pub fn messages_table() -> Markup {
    html! {
        form hx-get="/messages" hx-target="#parent-div" hx-swap="innerHTML" method="get" {
        }
    }
}
