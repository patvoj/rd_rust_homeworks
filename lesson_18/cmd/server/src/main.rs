use anyhow::{self, Ok};
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use templates::index::index;

use db::db_init;
use messages::handler::{create_message, get_all_messages};
use messages::repository::{AppState, MessageRepository};
use shared::app_metrics::AppMetrics;
use shared::AppState;

mod db;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Set up the Postgres connection pool
    let pool = db_init().await?;

    // Set up Prometheus metrics
    let metrics = Arc::new(AppMetrics::initialize());

    // Create the repository and wrap it in shared app state
    let repo = Arc::new(MessageRepository::new(pool));
    let app_state = AppState {
        repo,
        metrics: metrics.clone(),
    };

    // Build the router
    let app = Router::new()
        .route("/", get(index))
        .route("/metrics", get(AppMetrics::metrics)) // Use the imported AppMetrics
        .route("/messages", get(get_all_messages))
        .route("/messages", post(create_message))
        .with_state(app_state);

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}
