use crate::model::MessageType;
use crate::repository::AppState;
use axum::{extract::State, http::StatusCode, Json};

pub async fn get_all_messages(
    State(state): State<AppState>,
) -> Result<Json<Vec<MessageType>>, StatusCode> {
    match state.repo.get_all_messages().await {
        Ok(messages) => Ok(Json(messages)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_message(
    State(state): State<AppState>,
    Json(payload): Json<MessageType>,
) -> StatusCode {
    match state.repo.create_message(&payload).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
