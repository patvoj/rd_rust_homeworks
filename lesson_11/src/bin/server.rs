use log::{error, info};
use std::collections::HashMap;
use std::env;
use std::io::Read;
use std::net::SocketAddr;
use std::net::{TcpListener, TcpStream};

use lesson_11::{deserialize_message, MessageType};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Default to localhost:11111 if no arguments provided
    let host = args.get(1).map_or("127.0.0.1".to_string(), |s| s.clone());
    let port = args.get(2).map_or("11111".to_string(), |s| s.clone());
    let address = format!("{}:{}", host, port);

    info!("Starting server on {}", address);
    run_server(&address);
}

fn run_server(address: &str) {
    let listener = TcpListener::bind(address).unwrap();
    let mut clients: HashMap<SocketAddr, TcpStream> = HashMap::new();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let addr = stream.peer_addr().unwrap();
                clients.insert(addr, stream.try_clone().unwrap());

                // Handle the client in a separate thread
                std::thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                error!("Failed to accept connection: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut len_bytes = [0u8; 4];
    if stream.read_exact(&mut len_bytes).is_err() {
        error!("Failed to read message length.");
        return;
    };

    // Convert received bytes into message length
    let len = u32::from_be_bytes(len_bytes) as usize;
    let mut buffer = vec![0u8; len];

    if stream.read_exact(&mut buffer).is_err() {
        error!("Failed to read message data.");
        return;
    }

    // Deserialize the received message and handle different types
    match deserialize_message(&buffer) {
        MessageType::Text(msg) => println!("Received: {}", msg),
        MessageType::Image(_) => println!("Received an image"),
        MessageType::File(name, _) => println!("Received file: {}", name),
    }
}
