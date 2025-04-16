use anyhow::{Context, Result};
use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;

pub async fn db_init() -> Result<Pool<Postgres>> {
    println!("Attempting to connect to the database...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect("postgres://admin:adminpassword@localhost/rd-rust-db")
        .await
        .context("Failed to connect to the database.")?;

    println!("Connected to the database succesfully.");

    const QUERY: &str = "
    CREATE TABLE IF NOT EXISTS messages (
        id BIGSERIAL PRIMARY KEY,
        message JSONB NOT NULL
    );";

    // Create messages table
    sqlx::query(QUERY)
        .execute(&pool)
        .await
        .context("Failed to create the 'messages' table.")?;

    Ok(pool)
}
