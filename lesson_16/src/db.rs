use crate::MessageType;
use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;
use std::io;
use std::io::Write;

/// Initializes the PostgreSQL connection pool and creates necessary tables.
/// Also inserts a default admin user if it doesn't exist.
pub async fn db_init() -> Result<Pool<Postgres>, sqlx::Error> {
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

/// Saves a message to the database if it does not already exist.
///
/// # Arguments
/// * `pool` - The database connection pool.
/// * `message` - The message to be saved.
///
/// # Returns
/// * The ID of the saved message.
pub async fn save_to_db(
    pool: &Pool<Postgres>,
    message: &MessageType,
) -> Result<(i64,), sqlx::Error> {
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

/// Authenticates a user using the provided credentials against the `users` table.
///
/// # Arguments
/// * `pool` - The database connection pool.
/// * `username` - The username to authenticate.
/// * `password` - The password for the given username.
///
/// # Returns
/// * `true` if authentication is successful, otherwise `false`.
pub async fn authenticate_user(pool: &Pool<Postgres>, username: &str, password: &str) -> bool {
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

/// Prompts the user to enter a username and password via the terminal.
///
/// # Returns
/// * A tuple containing the entered `(username, password)`.
pub async fn get_credentials() -> (String, String) {
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
