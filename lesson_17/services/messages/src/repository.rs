use crate::model::MessageType;
use sqlx::Row;
use sqlx::{Error, PgPool};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<MessageRepository>,
}
pub struct MessageRepository {
    pool: PgPool,
}

impl MessageRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_message(&self, message: &MessageType) -> Result<i64, Error> {
        let json_value =
            serde_json::to_value(message).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        println!("Storing JSON: {}", json_value);

        const QUERY: &str = "
            INSERT INTO messages (message)
            VALUES ($1)
            RETURNING id;
        ";

        let row: (i64,) = sqlx::query_as(QUERY)
            .bind(json_value)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.0)
    }

    pub async fn get_all_messages(&self) -> Result<Vec<MessageType>, Error> {
        const QUERY: &str = "SELECT message FROM messages";

        let rows = sqlx::query(QUERY).fetch_all(&self.pool).await?;

        let mut messages: Vec<MessageType> = Vec::new();

        for row in rows {
            let json_value: serde_json::Value = row.try_get("message")?;

            println!("Retrieved JSON: {}", json_value);

            let message = serde_json::from_value(json_value).map_err(|e| {
                println!("Deserialization error: {}", e);
                sqlx::Error::Decode(Box::new(e))
            })?;

            messages.push(message);
        }

        Ok(messages)
    }
}
