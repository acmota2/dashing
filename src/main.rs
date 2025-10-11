mod config;
mod routes;

use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use config::{AppState, ServerConfig};
use minijinja::Environment;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let mut environment = Environment::new();
    minijinja_embed::load_templates!(&mut environment);
    let assets = ServeDir::new("assets");

    let config = match ServerConfig::new().populate(Some("./config.json")) {
        Ok(cfg) => cfg,
        Err(err) => panic!("Could not open the configuration file: {:?}", err),
    };

    let app = Router::new()
        .route("/", get(routes::root))
        .route("/utils", get(routes::icons))
        .route("/links", get(routes::links))
        .route("/settings", get(routes::settings))
        .route("/save-settings", post(routes::save_settings))
        .nest_service("/assets", assets)
        .with_state(Arc::new(AppState::new(environment, config)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
