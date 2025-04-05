use anyhow::Result;
use log::info;
use std::env;
use std::io::{self, Write};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::task;

use lesson_16::MessageType;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // Default to localhost:11111 if no arguments provided
    let host = args.get(1).map_or("127.0.0.1".to_string(), |s| s.clone());
    let port = args.get(2).map_or("11111".to_string(), |s| s.clone());
    let address = format!("{}:{}", host, port);

    info!("Starting client connecting to {}", address);
    run_client(&address).await?;
    Ok(())
}

/// Runs the client loop, handling user input and sending/receiving messages.
///
/// # Arguments
/// * `address` - The server address to connect to (e.g., "127.0.0.1:11111").
async fn run_client(address: &str) -> Result<()> {
    let stream = TcpStream::connect(address).await?;
    let (mut reader, mut writer) = stream.into_split();

    let reader_handle = task::spawn(async move {
        loop {
            let msg = MessageType::receive(&mut reader).await;

            match msg {
                Ok(MessageType::Text(text)) => println!("Received: {text}"),
                Ok(MessageType::Image(_)) => println!("Received an image"),
                Ok(MessageType::File(name, _)) => println!("Received file: {name}"),
                Err(e) => {
                    eprintln!("Error receiving message: {e}");
                    break;
                }
            }
        }
    });

    loop {
        print!("Enter message: ");
        io::stdout().flush().unwrap(); // Ensure prompt is displayed before input

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            info!("Exiting...");
            writer.shutdown().await?;
            break;
        }

        // Create a message and send it to the server
        let message = MessageType::Text(input.to_string());
        if let Err(e) = message.send(&mut writer).await {
            eprintln!("Error sending message: {e}");
            break;
        }
    }

    reader_handle.await?;
    Ok(())
}
