mod routes;

use std::sync::Arc;

use axum::{Router, routing::get};
use minijinja::Environment;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let mut env = Environment::new();
    minijinja_embed::load_templates!(&mut env);
    let assets = ServeDir::new("assets");

    let app = Router::new()
        .route("/", get(routes::root))
        .route("/utils", get(routes::icons))
        .route("/links", get(routes::links))
        .nest_service("/assets", assets)
        .with_state(Arc::new(env));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
