use axum::Router;

use crate::state::AppState;

pub mod game;
pub mod health;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/api/health", health::router())
        .nest("/api/games", game::router())
}
