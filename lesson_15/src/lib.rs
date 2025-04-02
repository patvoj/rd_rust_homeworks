use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Text(String),
    Image(Vec<u8>),
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
            // return Err(anyhow!(...));
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
