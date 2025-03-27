use log::{error, info};
use std::env;
use std::io::{self, Write};
use std::net::TcpStream;
use std::thread;

use lesson_11::MessageType;

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
    let mut stream = TcpStream::connect(address).unwrap();
    let mut stream_clone = TcpStream::try_clone(&stream).unwrap();

    let reader_handle = thread::spawn(move || {
        let mut stream = stream_clone;
        loop {
            let msg = MessageType::receive(&mut stream);

            if let Err(e) = msg {
                error!("error: {e}");
                break;
            };

            let msg = msg.unwrap();

            match msg {
                MessageType::File(_, _) => todo!(),
                MessageType::Image(_) => todo!(),
                MessageType::Text(text) => println!("{text}"),
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
            break;
        }

        // Create a message and send it to the server
        let message = MessageType::Text(input.to_string());
        message.send(&mut stream); // handle error
    }

    reader_handle.join();
}
