use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum MessageType {
    Text(String),
    Image(String),
    File(FileData),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FileData {
    pub filename: String,
    pub content: String, // Typically base64-encoded
}
