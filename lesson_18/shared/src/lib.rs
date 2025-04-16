pub mod app_metrics;
pub mod model;

use crate::app_metrics::AppMetrics;
use async_trait::async_trait;
use model::MessageType;
use std::sync::Arc;

#[async_trait]
pub trait MessageRepositoryTrait: Send + Sync + 'static {
    async fn create_message(&self, message: &MessageType) -> Result<i64, sqlx::Error>;
    async fn get_all_messages(&self) -> Result<Vec<MessageType>, sqlx::Error>;
}

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn MessageRepositoryTrait>,
    pub metrics: Arc<AppMetrics>,
}
