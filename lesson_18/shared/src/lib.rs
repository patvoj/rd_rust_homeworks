pub mod app_metrics;
pub mod model;

use crate::app_metrics::AppMetrics;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait MessageRepositoryTrait: Send + Sync + 'static {
    async fn create_message(&self, message: &serde_json::Value) -> Result<i64, sqlx::Error>;
    async fn get_all_messages(&self) -> Result<Vec<serde_json::Value>, sqlx::Error>;
}

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn MessageRepositoryTrait>,
    pub metrics: Arc<AppMetrics>,
}
