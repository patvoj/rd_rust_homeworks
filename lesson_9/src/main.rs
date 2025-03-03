use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::io::{self, Read};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::Path;
use std::thread;

#[derive(Serialize, Deserialize, Debug)]
enum MessageType {
    Text(String),
    Image { data: Vec<u8> },
    File { name: String, data: Vec<u8> },
}

fn serialize_message(message: &MessageType) -> String {
    serde_json::to_string(&message).unwrap()
}

fn deserialize_message(data: &[u8]) -> MessageType {
    serde_json::from_slice(&data).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <IP> <PORT> <server|client>", args[0]);
        return;
    }

    let address = format!("{}:{}", &args[1], &args[2]);

    if args[3] == "server" {
        println!("Starting server on {}", address);
        run_server(&address);
    } else {
        println!("Starting client on {}", address);
        run_client(&address);
    }
}

fn run_server(address: &str) {
    let listener = TcpListener::bind(address).expect("Failed to bind to address.");

    let mut clients: HashMap<SocketAddr, TcpStream> = HashMap::new();

    println!("Server listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let addr = stream.peer_addr().expect("Could not get peer address.");
                println!("New client connected: {}", addr);

                clients.insert(addr, stream.try_clone().expect("Failed to clone stream."));

                thread::spawn(move || {
                    let message = handle_client(stream, addr);
                    println!("{:?}", message);
                });
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, addr: SocketAddr) -> io::Result<MessageType> {
    println!("Handling client: {}", addr);

    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes)?;

    let len = u32::from_be_bytes(len_bytes) as usize;
    let mut buffer = vec![0u8; len];

    stream.read_exact(&mut buffer)?;

    Ok(deserialize_message(&buffer))
}

fn run_client(address: &str) {
    println!("Connected to server at {}", address);
    println!("Enter message type (text, image, file):");

    let address = address.to_string();

    thread::spawn(move || {
        let mut stream = match TcpStream::connect(&address) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to connect to server: {}", e);
                return;
            }
        };

        println!("Connected to {}", address);

        loop {
            println!("Enter message type (text, image, file):");
            print!("> ");

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                println!("Error reading input.");
                continue;
            }

            let input = input.trim();

            let message = match input {
                "text" => {
                    println!("Enter text message:");
                    let mut text = String::new();
                    io::stdin().read_line(&mut text).unwrap();
                    MessageType::Text(text.trim().to_string())
                }
                "file" => {
                    println!("Enter file path:");
                    let mut file_path = String::new();
                    io::stdin().read_line(&mut file_path).unwrap();
                    let file_path = file_path.trim();

                    match fs::read(file_path) {
                        Ok(data) => {
                            let name = Path::new(file_path)
                                .file_name()
                                .unwrap()
                                .to_string_lossy()
                                .into_owned();
                            MessageType::File { name, data }
                        }
                        Err(e) => {
                            println!("Failed to read file: {}", e);
                            continue;
                        }
                    }
                }
                _ => {
                    println!("Unknown message type. Use 'text' or 'file'.");
                    continue;
                }
            };

            if let Err(e) = send_message(&address, &message) {
                println!("Failed to send message: {}", e);
                break;
            }
        }
    });
}

fn send_message(address: &str, message: &MessageType) -> io::Result<()> {
    let serialized = serialize_message(message);
    let mut stream = TcpStream::connect(address)?;

    // Send the length of the serialized message (as 4-byte value)
    let len = serialized.len() as u32;
    stream.write_all(&len.to_be_bytes())?;

    // Send the serialized message
    stream.write_all(serialized.as_bytes())?;

    Ok(())
}
