# Rust TCP Client-Server Application

This is a simple TCP client-server application written in Rust. The client can send messages to the server, and the server processes and prints the received messages.

## Features

- Supports text message transmission
- Handles multiple client connections
- Serializes and deserializes messages
- Graceful handling of client exits

## Requirements

- Rust (latest stable version recommended)
- `log` crate for logging

## Installation

1. Clone the repository:
   ```sh
   git clone git@github.com:patvoj/rd_rust_homeworks.git
   cd lesson_11
   ```
2. Ensure Rust is installed:
   ```sh
   rustc --version
   ```
3. Build the project:
   ```sh
   cargo build --release
   ```

## Usage

### Start the Server

Run the server with default settings (localhost:11111):

```sh
cargo run --bin server
```

Or specify a custom host and port:

```sh
cargo run --bin server -- 192.168.1.100 5000
```

### Start the Client

Run the client and connect to the server:

```sh
cargo run --bin client
```

Or specify a custom server address:

```sh
cargo run --bin client -- 192.168.1.100 5000
```

### Sending Messages

- After starting the client, type a message and press Enter to send it.
- Type `exit` to close the client.

## Code Overview

### `client.rs`

- Connects to the server via TCP.
- Reads user input and sends serialized messages.
- Supports graceful exit on user command.

### `server.rs`

- Listens for incoming TCP connections.
- Handles multiple clients concurrently.
- Receives, deserializes, and prints messages.

## Dependencies

This project depends on:

```toml
[dependencies]
log = "0.4"
lesson_11 = { path = "./lesson_11" }
```
