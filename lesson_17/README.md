# 📬 Axum Message Service

A small, modular message service built with Rust using the Axum framework, SQLx for database interaction, and HTMX + Maud for a dynamic frontend experience.

## ✨ Features

🧩 Modular architecture with clear separation of concerns\
🌐 RESTful API with Axum\
🗃️ PostgreSQL database for message storage\
⚡ Async operations using Tokio\
💬 JSON-based message types: text, image, and file\
🧠 Server-side rendering using Maud\
🔁 Interactive frontend with HTMX

## 📦 Installation

### ✅ Prerequisites

Rust & Cargo
Docker (for PostgreSQL)
PostgreSQL CLI tools (optional, for DB inspection)

Ensure you have the following installed:

- Rust & Cargo ([Install Rust](https://www.rust-lang.org/tools/install))
- PostgreSQL ([Install PostgreSQL](https://www.postgresql.org/download/))

### 🚀 Clone the Repository

```sh
$ git clone https://github.com/patvoj/rd_rust_homeworks.git
$ cd lesson_17
```

### 🐘 Start PostgreSQL

1. Run PostgreSQL

```
docker-compose up -d
```

DB URL: postgres://admin:adminpassword@localhost/rd-rust-db

2. 🦀 Run the Axum Server

```
cd cmd/server
cargo run
```

App runs at http://localhost:3000

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

## 📬 API Endpoints

| Method | Path        | Request Body (Example)                                                                                                                                                                            | Description            |
| ------ | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------- |
| GET    | `/messages` | _None_                                                                                                                                                                                            | Retrieve all messages  |
| POST   | `/messages` | `{ "type": "Text", "data": "Hello!" }` <br> `{ "type": "Image", "data": "https://example.com/image.png" }` <br> `{ "type": "File", "data": { "filename": "notes.txt", "content": "base64..." } }` | Submit a new message   |
| GET    | `/`         | _None_                                                                                                                                                                                            | Load the HTML frontend |

### 🖥 Frontend

Built using HTMX for dynamic behavior
Rendered with Maud (Rust-based HTML templating)

Includes:\
📜 Message list\
📝 Message submission form

## 📁 Project Structure

```
.
├── cmd/
│   └── server/               # Application entry point
│       └── src/
│           ├── main.rs       # Axum router and server startup
│           └── db.rs         # DB connection and setup
├── services/
│   └── messages/
│       └── src/
│           ├── handler.rs    # Route handler logic
│           ├── model.rs      # MessageType enum
│           ├── repository.rs # DB interactions (Repo pattern)
│           └── service.rs    # Message service wrapper
├── templates/
│   └── src/
│       ├── index.rs          # HTML page
│       ├── message_form.rs   # Form and table (Maud templates)
│       └── lib.rs
├── docker-compose.yml        # PostgreSQL setup
├── README.md
```

## 🛠 Dependencies

- tokio – async runtime
- axum – web framework
- sqlx – DB layer
- serde – serialization
- anyhow – error handling
- maud – server-side HTML rendering
- htmx – frontend interactivity

## 🔮 Future Improvements

🔐 Add authentication (JWT or sessions)\
🔏 Password hashing & user registration\
📬 Private and group messaging support\
💅 Enhanced frontend UI/UX with styles or a JS framework\
📁 File upload support
