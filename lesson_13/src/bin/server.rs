use log::{error, info};
use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

use lesson_13::MessageType;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Default to localhost:11111 if no arguments provided
    let host = args.get(1).map_or("127.0.0.1".to_string(), |s| s.clone());
    let port = args.get(2).map_or("11111".to_string(), |s| s.clone());
    let address = format!("{}:{}", host, port);

    info!("Starting server on {}", address);
    run_server(&address);
}

type Clients = Arc<Mutex<HashMap<SocketAddr, TcpStream>>>;

fn run_server(address: &str) {
    let listener = TcpListener::bind(address).unwrap();
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let addr = stream.peer_addr().unwrap();

                {
                    let mut lock = clients.lock().unwrap();
                    lock.insert(addr, stream.try_clone().unwrap());
                } // <-- lock dropped

                // Handle the client in a separate thread
                let clients_clone = Arc::clone(&clients);
                std::thread::spawn(move || {
                    handle_client(stream, addr, clients_clone);
                });
            }
            Err(e) => {
                error!("Failed to accept connection: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, addr: SocketAddr, clients: Clients) {
    loop {
        let msg = MessageType::receive(&mut stream);

        match &msg {
            Ok(MessageType::Text(msg)) => println!("Received: {}", msg),
            Ok(MessageType::Image(_)) => println!("Received an image"),
            Ok(MessageType::File(name, _)) => println!("Received file: {}", name),
            Err(e) => {
                error!("error: {e}");
                return;
            }
        }

        let msg = msg.unwrap(); // always ok

        let mut clients_lock = clients.lock().unwrap();

        let mut clients_to_remove = vec![];
        for (client_addr, mut client_stream) in clients_lock.iter_mut() {
            if *client_addr == addr {
                continue;
            }

            if let Err(e) = msg.send(&mut client_stream) {
                info!("Failed to send message to {client_addr}, error: {e}. Closing...");
                clients_to_remove.push(client_addr.clone());
            }
        }

        for client in clients_to_remove {
            clients_lock.remove(&client);
        }
    }
}
