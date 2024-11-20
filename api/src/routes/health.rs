use axum::{routing::get, Router};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(health_handler))
}

pub async fn health_handler() -> &'static str {
    "ok"
}
