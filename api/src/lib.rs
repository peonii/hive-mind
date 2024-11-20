use state::AppState;

pub mod models;
pub mod routes;
pub mod state;

pub async fn run() {
    tracing_subscriber::fmt().init();

    let port = std::env::var("PORT").unwrap_or("3001".to_owned());

    let state = AppState::new();

    let router = routes::router().with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind to port");

    axum::serve(listener, router)
        .await
        .expect("Failed to serve app");
}
