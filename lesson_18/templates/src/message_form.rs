use maud::{html, Markup};

pub fn send_message_form() -> Markup {
    html! {
        form id="message-form" {
            input type="text" id="message-input" placeholder="Type your message..." required;
            button type="submit" { "Send" }
        }

        script {
            (maud::PreEscaped(r#"
                document.getElementById('message-form').addEventListener('submit', async function(e) {
                    e.preventDefault();
                    const input = document.getElementById('message-input');
                    const payload = {
                        type: "Text",
                        data: input.value
                    };

                    const response = await fetch('/messages', {
                        method: 'POST',
                        headers: {
                            'Content-Type': 'application/json',
                            'HX-Request': 'true'
                        },
                        body: JSON.stringify(payload)
                    });

                    if (response.ok) {
                        // Refresh the parent div content
                        const html = await fetch('/').then(res => res.text());
                        document.getElementById('parent-div').innerHTML = html;
                        input.value = "";
                    } else {
                        alert("Failed to send message.");
                    }
                });
            "#))
        }
    }
}
