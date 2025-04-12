use anyhow::{bail, Result};
use base64::Engine;
use serde::de::{Deserializer, Error};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum MessageType {
    Text(String),
    #[serde(deserialize_with = "decode_base64")]
    Image(Vec<u8>),
    #[serde(deserialize_with = "decode_base64_file")]
    File(String, Vec<u8>),
}

impl MessageType {
    pub fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn deserialize(data: &[u8]) -> Result<Self> {
        Ok(serde_json::from_slice(&data)?)
    }

    pub async fn receive(stream: &mut OwnedReadHalf) -> Result<Self> {
        let mut len_bytes = [0u8; 4];
        if stream.read_exact(&mut len_bytes).await.is_err() {
            bail!("Failed to read message length.");
        };

        // Convert received bytes into message length
        let len = u32::from_be_bytes(len_bytes) as usize;
        let mut buffer = vec![0u8; len];

        if stream.read_exact(&mut buffer).await.is_err() {
            bail!("Failed to read message data.");
        }

        Self::deserialize(&buffer)
    }

    pub async fn send(&self, stream: &mut OwnedWriteHalf) -> Result<()> {
        let serialized = self.serialize();

        let len = serialized.len() as u32;
        stream.write(&len.to_be_bytes()).await.unwrap();

        // Send the serialized message
        stream.write_all(&serialized.as_bytes()).await.unwrap();

        Ok(())
    }
}

fn decode_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let base64_str: String = Deserialize::deserialize(deserializer)?;
    base64::engine::general_purpose::STANDARD
        .decode(&base64_str)
        .map_err(D::Error::custom)
}

fn decode_base64_file<'de, D>(deserializer: D) -> Result<(String, Vec<u8>), D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct FileData {
        filename: String,
        content: String,
    }

    let file_data: FileData = Deserialize::deserialize(deserializer)?;
    let content = base64::engine::general_purpose::STANDARD
        .decode(&file_data.content)
        .map_err(D::Error::custom)?;
    Ok((file_data.filename, content))
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_message_serialization_text() -> Result<()> {
        let msg = MessageType::Text("Hello, world!".to_string());
        let serialized = msg.serialize();
        let deserialized = MessageType::deserialize(serialized.as_bytes())?;

        assert_eq!(msg, deserialized);
        Ok(())
    }

    #[test]
    fn test_message_serialization_image() -> Result<()> {
        let data = vec![1, 2, 3, 4, 5];
        let msg = MessageType::Image(data.clone());
        let serialized = msg.serialize();
        let deserialized = MessageType::deserialize(serialized.as_bytes())?;

        assert_eq!(msg, deserialized);
        Ok(())
    }

    #[test]
    fn test_message_serialization_file() -> Result<()> {
        let filename = "file.txt".to_string();
        let contents = vec![10, 20, 30];
        let msg = MessageType::File(filename.clone(), contents.clone());
        let serialized = msg.serialize();
        let deserialized = MessageType::deserialize(serialized.as_bytes())?;

        assert_eq!(msg, deserialized);
        Ok(())
    }
}
