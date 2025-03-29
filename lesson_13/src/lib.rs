use std::io::{Read, Write};
use std::net::TcpStream;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

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

    pub fn receive(stream: &mut TcpStream) -> Result<Self> {
        let mut len_bytes = [0u8; 4];
        if stream.read_exact(&mut len_bytes).is_err() {
            bail!("Failed to read message length.");
            // return Err(anyhow!(...));
        };

        // Convert received bytes into message length
        let len = u32::from_be_bytes(len_bytes) as usize;
        let mut buffer = vec![0u8; len];

        if stream.read_exact(&mut buffer).is_err() {
            bail!("Failed to read message data.");
        }

        Self::deserialize(&buffer)
    }

    pub fn send(&self, stream: &mut TcpStream) -> Result<()> {
        let serialized = self.serialize();

        let len = serialized.len() as u32;
        stream.write(&len.to_be_bytes()).unwrap();

        // Send the serialized message
        stream.write_all(&serialized.as_bytes()).unwrap();

        Ok(())
    }
}
