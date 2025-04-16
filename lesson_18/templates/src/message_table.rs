use maud::{html, Markup};
use messages::model::MessageType;

pub fn message_table(messages: &[MessageType]) -> Markup {
    html! {
        table border="1" {
            thead {
                tr {
                    th { "Type" }
                    th { "Content" }
                }
            }
            tbody {
                @for message in messages {
                    tr {
                        td {
                            @match message {
                                MessageType::Text(_) => { "Text" },
                                MessageType::Image(_) => { "Image" },
                                MessageType::File(_) => { "File" },
                            }
                        }
                        td {
                            @match message {
                                MessageType::Text(text) => {
                                    (text)
                                },
                                MessageType::Image(url) => {
                                    {
                                        img src=(url) alt="Image" style="max-height: 100px;" {}
                                    }
                                },
                                MessageType::File(file) => {
                                    {
                                        div {
                                            strong { (file.filename) }
                                            br;
                                            code { (file.content) }
                                        }
                                    }
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}
