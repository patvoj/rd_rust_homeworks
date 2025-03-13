use log::info;
use std::env;
use std::io::{self, Write};
use std::net::TcpStream;

use lesson_11::{serialize_message, MessageType};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Default to localhost:11111 if no arguments provided
    let host = args.get(1).map_or("127.0.0.1".to_string(), |s| s.clone());
    let port = args.get(2).map_or("11111".to_string(), |s| s.clone());
    let address = format!("{}:{}", host, port);

    info!("Starting client connecting to {}", address);
    run_client(&address);
}

fn run_client(address: &str) {
    info!("Connected to server. Type 'exit' to quit.");

    loop {
        print!("Enter message: ");
        io::stdout().flush().unwrap(); // Ensure prompt is displayed before input

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            info!("Exiting...");
            break;
        }

        // Create a message and send it to the server
        let message = MessageType::Text(input.to_string());
        send_message(address, &message);
    }
}

fn send_message(address: &str, message: &MessageType) {
    let serialized = serialize_message(message);
    let mut stream = TcpStream::connect(address).unwrap();

    // Send the length of the serialized message (as 4-byte value)
    let len = serialized.len() as u32;
    stream.write(&len.to_be_bytes()).unwrap();

    // Send the serialized message
    stream.write_all(&serialized.as_bytes()).unwrap();
}
