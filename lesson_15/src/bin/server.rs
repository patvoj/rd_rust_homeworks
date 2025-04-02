use anyhow::anyhow;
use anyhow::bail;
use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;
use std::collections::HashMap;
use std::env;
use std::io;
use std::io::Write;
use std::net::SocketAddr;
use std::result::Result::{Err, Ok};
use std::sync::Arc;
use tokio;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use lesson_15::MessageType;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();

    // Default to localhost:11111 if no arguments provided
    let host = args.get(1).map_or("127.0.0.1".to_string(), |s| s.clone());
    let port = args.get(2).map_or("11111".to_string(), |s| s.clone());
    let address = format!("{}:{}", host, port);

    println!("Starting server on {}", address);
    if let Err(e) = run_server(&address).await {
        return Err(anyhow!("Error occured: {}", e));
    };

    Ok(())
}

type Clients = Arc<Mutex<HashMap<SocketAddr, OwnedWriteHalf>>>;

async fn run_server(address: &str) -> Result<(), anyhow::Error> {
    let listener = TcpListener::bind(address).await?;
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let pool = match db_init().await {
        Ok(pool) => Arc::new(pool),
        Err(e) => {
            return Err(anyhow!(
                "Error occurred when establishing database pool: {}",
                e
            ));
        }
    };

    println!("Server listening on {}", address);

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                let (reader, writer) = stream.into_split();

                {
                    let mut lock = clients.lock().await;
                    lock.insert(addr, writer);
                } // lock dropped here

                // Handle the client in a separate task
                let clients_clone = Arc::clone(&clients);
                let pool_clone = Arc::clone(&pool);

                tokio::spawn(async move {
                    if let Err(e) = handle_client(reader, addr, clients_clone, pool_clone).await {
                        eprintln!("Error handling client {}: {}", addr, e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
                continue;
            }
        }
    }
}

async fn handle_client(
    mut reader: OwnedReadHalf,
    addr: SocketAddr,
    clients: Clients,
    pool: Arc<Pool<Postgres>>,
) -> Result<(), anyhow::Error> {
    let credentials = get_credentials().await;
    let is_authenticated = authenticate_user(&pool, &credentials.0, &credentials.1).await;
    if is_authenticated == false {
        bail!("Closing client...")
    }

    loop {
        let msg = MessageType::receive(&mut reader).await;

        match &msg {
            Ok(MessageType::Text(msg)) => println!("Received text: {}", msg),
            Ok(MessageType::Image(_)) => println!("Received an image"),
            Ok(MessageType::File(name, _)) => println!("Received file: {}", name),
            Err(e) => {
                return Err(anyhow!("Error: {e}"));
            }
        }

        let msg = msg.unwrap(); // always ok
        match save_to_db(&pool, &msg).await {
            Ok((id,)) => {
                println!("Message was saved under ID: {}", id);
            }
            Err(e) => {
                return Err(anyhow!("Failed to save message to database: {}", e));
            }
        }

        let mut clients_lock = clients.lock().await;
        let mut clients_to_remove = vec![];

        for (client_addr, mut client_stream) in clients_lock.iter_mut() {
            if *client_addr == addr {
                continue;
            }

            if let Err(e) = msg.send(&mut client_stream).await {
                clients_to_remove.push(client_addr.clone());
                bail!("Failed to send message to {client_addr}, error: {e}. Closing...");
            }
        }

        for client in clients_to_remove {
            clients_lock.remove(&client);
        }
    }
}

async fn db_init() -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://admin:adminpassword@localhost/rd-rust-db")
        .await?;

    // Create messages table
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS messages (
            id BIGSERIAL PRIMARY KEY,
            message TEXT NOT NULL
        );",
    )
    .execute(&pool)
    .await?;

    // Create users table
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS users (
            id BIGSERIAL PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            password TEXT NOT NULL
        );",
    )
    .execute(&pool)
    .await?;

    // Insert default user only if not exists
    sqlx::query(
        "
        INSERT INTO users (username, password)
        SELECT $1, $2
        WHERE NOT EXISTS (
            SELECT 1 FROM users WHERE username = $1
        );",
    )
    .bind("admin")
    .bind("password")
    .execute(&pool)
    .await?;

    Ok(pool)
}

async fn save_to_db(pool: &Pool<Postgres>, message: &MessageType) -> Result<(i64,), sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        "
        INSERT INTO messages (message)
        SELECT $1
        WHERE NOT EXISTS (
            SELECT 1 FROM messages WHERE message = $1
        )
        RETURNING id;
        ",
    )
    .bind(message.serialize())
    .fetch_one(pool)
    .await?;

    Ok(row)
}

async fn authenticate_user(pool: &Pool<Postgres>, username: &str, password: &str) -> bool {
    let user_exists = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM users WHERE username = $1 AND password = $2",
    )
    .bind(username)
    .bind(password)
    .fetch_one(pool)
    .await
    .unwrap_or(0);

    if user_exists > 0 {
        eprintln!("Authentication successful. Welcome, {}!", username);
        return true;
    } else {
        eprintln!("Authentication failed. Invalid username or password.");
        return false;
    }
}

async fn get_credentials() -> (String, String) {
    // Prompt for username
    println!("Enter username: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed before reading input

    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read username");
    let username = username.trim().to_string(); // Remove newline and whitespace

    // Prompt for password
    println!("Enter password: ");
    io::stdout().flush().unwrap();

    let mut password = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read password");
    let password = password.trim().to_string(); // Remove newline and whitespace

    (username, password)
}
