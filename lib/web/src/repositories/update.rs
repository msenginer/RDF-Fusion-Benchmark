use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use crate::AppState;

pub async fn handle_update_post(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: String,
) -> Response {
    if state.read_only {
        return (StatusCode::FORBIDDEN, "Read-only mode").into_response();
    }

    let query = if headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.contains("application/x-www-form-urlencoded"))
        .unwrap_or(false)
    {
        // Parse form-encoded: update=INSERT+DATA+...
        form_urlencoded::parse(body.as_bytes())
            .find(|(key, _)| key == "update")
            .map(|(_, value)| value.into_owned())
            .unwrap_or(body)
    } else {
        body
    };

    match state.store.update(&query).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Update error: {e}")).into_response(),
    }
}