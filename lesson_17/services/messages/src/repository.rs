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
        const QUERY: &str = "
            INSERT INTO messages (message)
            SELECT $1
            WHERE NOT EXISTS (
                SELECT 1 FROM messages WHERE message = $1
            )
            RETURNING id;
        ";

        let serialized_message = serde_json::to_string(&message).unwrap();

        let row: (i64,) = sqlx::query_as(QUERY)
            .bind(serialized_message)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.0)
    }

    pub async fn get_all_messages(&self) -> Result<Vec<MessageType>, Error> {
        const QUERY: &str = "SELECT message FROM messages";

        let rows = sqlx::query(QUERY).fetch_all(&self.pool).await?;

        let mut messages: Vec<MessageType> = Vec::new();

        for row in rows {
            let message: String = row.try_get("message")?;
            let deserialized_message: MessageType = serde_json::from_str(&message).unwrap();
            messages.push(deserialized_message);
        }

        Ok(messages)
    }
}
