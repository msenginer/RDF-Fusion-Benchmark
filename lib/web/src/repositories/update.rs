use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use crate::AppState;

pub async fn handle_update_post(
    State(state): State<AppState>,
    body: String,
) -> Response {
    if state.read_only {
        return (StatusCode::FORBIDDEN, "Read-only mode").into_response();
    }
    match state.store.update(&body).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Update error: {e}")).into_response(),
    }
}