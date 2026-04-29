use axum::extract::State;
use axum::http::StatusCode;
use crate::AppState;

pub async fn handle_update_post(
    State(state): State<AppState>,
    body: String,
) -> Result<StatusCode, StatusCode> {
    if state.read_only {
        return Err(StatusCode::FORBIDDEN);
    }
    state
        .store
        .update(&body)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}