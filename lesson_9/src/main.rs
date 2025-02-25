use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::io::Read;
use std::io::Write;
use std::net::SocketAddr;
use std::net::{TcpListener, TcpStream};

#[derive(Serialize, Deserialize, Debug)]
enum MessageType {
    Text(String),
    Image(Vec<u8>),
    File(String, Vec<u8>),
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // default to localhost:11111 if no arguments provided
    let host = args.get(1).map_or("127.0.0.1".to_string(), |s| s.clone());
    let port = args.get(2).map_or("11111".to_string(), |s| s.clone());
    let address = format!("{}:{}", host, port);

    if args.len() > 3 && args[3] == "--server" {
        println!("Starting server on {}", address);
        run_server(&address);
    } else {
        println!("Starting client connecting to {}", address);
        run_client(&address);
    }
}

fn run_server(address: &str) {
    let listener = TcpListener::bind(address).unwrap();

    let mut clients: HashMap<SocketAddr, TcpStream> = HashMap::new();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let addr = stream.peer_addr().unwrap();
                clients.insert(addr, stream.try_clone().unwrap());

                std::thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}

fn run_client(address: &str) {
    let message = MessageType::Text("Hello, server!".to_string());
    send_message(address, &message);
}

fn send_message(address: &str, message: &MessageType) {
    let serialized = serialize_message(message);
    let mut stream = TcpStream::connect(address).unwrap();

    // send the length of the serialized message (as 4-byte value).
    let len = serialized.len() as u32;
    stream.write(&len.to_be_bytes()).unwrap();

    // send the serialized message.
    stream.write_all(serialized.as_bytes()).unwrap();
}

fn handle_client(mut stream: TcpStream) -> MessageType {
    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes).unwrap();

    let len = u32::from_be_bytes(len_bytes) as usize;
    let mut buffer = vec![0u8; len];

    stream.read_exact(&mut buffer).unwrap();
    deserialize_message(&buffer)
}

fn serialize_message(message: &MessageType) -> String {
    serde_json::to_string(&message).unwrap()
}

fn deserialize_message(data: &[u8]) -> MessageType {
    serde_json::from_slice(&data).unwrap()
}
