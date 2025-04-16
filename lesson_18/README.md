# 📬 Axum Message Service

A small, modular message service built with Rust using the Axum framework, SQLx for database interaction, and HTMX + Maud for a dynamic frontend experience.

## ✨ Features

🧩 Modular architecture with clear separation of concerns  
🌐 RESTful API with Axum  
🗃️ PostgreSQL database for message storage  
⚡ Async operations using Tokio  
📊 Prometheus metrics endpoint (`/metrics`)  
💬 JSON-based message types: text, image, and file  
🧠 Server-side rendering using Maud  
🔁 Interactive frontend with HTMX

## 📦 Installation

### ✅ Prerequisites

- Rust (latest stable)
- Docker (for Postgres if needed)
- Prometheus (optional, for scraping metrics)

Ensure you have the following installed:

- Rust & Cargo ([Install Rust](https://www.rust-lang.org/tools/install))
- PostgreSQL ([Install PostgreSQL](https://www.postgresql.org/download/))

### 🚀 Clone the Repository

```sh
$ git clone https://github.com/patvoj/rd_rust_homeworks.git
$ cd lesson_18
```

### 🐘 Start PostgreSQL

1. Run PostgreSQL

```
docker-compose up -d
```

DB URL: postgres://admin:adminpassword@localhost/rd-rust-db

2. 🦀 Run the Axum Server

```sh
$ cd lesson_18
$ cargo run
```

App runs at http://localhost:3000

## 📬 API Endpoints

| Method | Path        | Request Body (Example)                                                                                                                                                                            | Description                |
| ------ | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------- |
| GET    | `/messages` | _None_                                                                                                                                                                                            | Retrieve all messages      |
| POST   | `/messages` | `{ "type": "Text", "data": "Hello!" }` <br> `{ "type": "Image", "data": "https://example.com/image.png" }` <br> `{ "type": "File", "data": { "filename": "notes.txt", "content": "base64..." } }` | Submit a new message       |
| GET    | `/`         | _None_                                                                                                                                                                                            | Load the HTML frontend     |
| GET    | `/metrics`  | _None_                                                                                                                                                                                            | Prometheus scrape endpoint |

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
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs       # Axum router and server startup
│           └── db.rs         # DB connection and setup
├── services/
│   └── messages/
│       ├── Cargo.toml
│       └── src/
│           ├── handler.rs    # Route handler logic
│           ├── lib.rs
│           ├── repository.rs # DB interactions (Repo pattern)
│           └── service.rs    # Message service wrapper
├── shared/
│   ├── Cargo.toml
│   └── src/
│       ├── app_metrics.rs    # Prometheus metrics logic
│       ├── lib.rs
│       └── model.rs          # Shared types and models
├── templates/
│   ├── Cargo.toml
│   └── src/
│       ├── index.rs          # Main page HTML rendering
│       ├── lib.rs
│       ├── message_form.rs   # Message input form
│       └── message_table.rs  # Message list/table
├── docker-compose.yml        # PostgreSQL setup
├── README.md
├── .gitignore
├── Cargo.toml                # Workspace root manifest
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
