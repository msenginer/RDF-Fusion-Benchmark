use crate::AppState;
use crate::repositories::data::handle_data_post;
use crate::repositories::query::handle_query_get;
use crate::repositories::update::handle_update_post;
use axum::Router;
use axum::routing::{get, post};

mod content_negotiation;
mod data;
mod query;
mod service_description;
mod sparql_query_params;
mod update;

pub fn create_repositories_routes() -> Router<AppState> {
    Router::new()
        .route("/default/query", get(handle_query_get))
        .route("/default/data", post(handle_data_post))
        .route("/default/update", post(handle_update_post))
}