use crate::model::MessageType;
use axum::{extract::State, http::StatusCode, Json};
use shared::app_metrics::AppMetrics;
use std::time::Instant;

// Assuming AppState is defined in cmd/server/src/main.rs as:
// #[derive(Clone)]
// pub struct AppState {
//     pub repo: Arc<MessageRepository>,
//     pub metrics: Arc<AppMetrics>,
// }

pub async fn get_all_messages(
    State(state): State<crate::AppState>, // Use crate::AppState to be explicit
) -> Result<Json<Vec<MessageType>>, StatusCode> {
    state.metrics.api_calls_total.inc();
    let start = Instant::now();
    let result = state.repo.get_all_messages().await;
    let duration = start.elapsed().as_secs_f64();
    state
        .metrics
        .request_latency_seconds
        .with_label_values("GET", "/messages")
        .observe(duration);
    match result {
        Ok(messages) => Ok(Json(messages)),
        Err(err) => {
            eprintln!("Failed to get messages: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_message(
    State(state): State<crate::AppState>,
    Json(payload): Json<MessageType>,
) -> StatusCode {
    state.metrics.api_calls_total.inc();
    let start = Instant::now();
    let result = state.repo.create_message(&payload).await;
    let duration = start.elapsed().as_secs_f64();
    state
        .metrics
        .request_latency_seconds
        .with_label_values("POST", "/messages")
        .observe(duration);
    match result {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
