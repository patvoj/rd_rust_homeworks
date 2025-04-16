use axum::{extract::State, http::StatusCode, Json};
use shared::model::MessageType;
use shared::AppState;
use std::time::Instant;

pub async fn get_all_messages(
    State(state): State<AppState>,
) -> Result<Json<Vec<MessageType>>, StatusCode> {
    state.metrics.api_calls_total.inc();
    let start = Instant::now();
    let result = state.repo.get_all_messages().await;
    let duration = start.elapsed().as_secs_f64();
    state
        .metrics
        .request_latency_seconds
        .with_label_values(&["GET", "/messages"])
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
    State(state): State<AppState>,
    Json(payload): Json<MessageType>,
) -> StatusCode {
    state.metrics.api_calls_total.inc();
    let start = Instant::now();
    let result = state.repo.create_message(&payload).await;
    let duration = start.elapsed().as_secs_f64();
    state
        .metrics
        .request_latency_seconds
        .with_label_values(&["POST", "/messages"])
        .observe(duration);
    match result {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
