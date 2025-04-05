# Rust TCP Chat Server & Client

This project is a simple TCP-based chat server and client built using Rust and Tokio for asynchronous networking. The server supports multiple clients and includes basic user authentication with a PostgreSQL database.

## Features

- **TCP server** that handles multiple clients concurrently.
- **Client application** to send and receive messages.
- **Message types**: Supports text, images, and files.
- **User authentication** using PostgreSQL.
- **Database persistence** for storing messages and user credentials.
- **Asynchronous operations** with Tokio.

## Installation

### Prerequisites

Ensure you have the following installed:

- Rust & Cargo ([Install Rust](https://www.rust-lang.org/tools/install))
- PostgreSQL ([Install PostgreSQL](https://www.postgresql.org/download/))

### Clone Repository

```sh
$ git clone https://github.com/patvoj/rd_rust_homeworks.git
$ cd lesson_16
```

### Configure Database

1. Start PostgreSQL and create a database:

```sh
$ psql -U postgres
postgres=# CREATE DATABASE rd-rust-db;
postgres=# \\q
```

2. Update the database connection string in `db_init()` function inside `server.rs`:

```rust
.connect("postgres://admin:adminpassword@localhost/rd-rust-db")
```

### Build and Run

#### Start the Server

```sh
$ cargo run --bin server
```

By default, it runs on `127.0.0.1:11111`.

#### Start a Client

```sh
$ cargo run --bin client
```

The client connects to `127.0.0.1:11111` by default.

## Usage

### Server

- Starts and listens for incoming client connections.
- Authenticates users and stores messages in the database.
- Broadcasts messages to all connected clients.

### Client

- Connects to the server.
- Prompts for username and password.
- Sends messages and receives messages from other clients.
- Type `exit` to disconnect.

## File Structure

```
├── src
│   ├── main.rs         # Server entry point
│   ├── client.rs       # Client entry point
│   ├── lib.rs          # MessageType handling
│   ├── server.rs       # Server logic and database operations
│   └── database.rs     # Database connection and queries
│
├── Cargo.toml          # Rust dependencies
├── README.md           # Project documentation
```

## Dependencies

- `tokio` - Asynchronous runtime.
- `serde` - Serialization and deserialization.
- `sqlx` - PostgreSQL support.
- `anyhow` - Error handling.
- `log` - Logging.

## Improvements & Future Enhancements

- Add encryption for secure message transmission.
- Implement a proper user registration and password hashing mechanism.
- Enhance UI with a GUI client.
- Implement private messaging.
