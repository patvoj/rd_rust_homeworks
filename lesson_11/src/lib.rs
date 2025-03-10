use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Text(String),
    Image(Vec<u8>),
    File(String, Vec<u8>),
}

pub fn serialize_message(message: &MessageType) -> String {
    serde_json::to_string(&message).unwrap()
}

pub fn deserialize_message(data: &[u8]) -> MessageType {
    serde_json::from_slice(&data).unwrap()
}
