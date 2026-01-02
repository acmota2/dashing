mod config;
mod routes;

use axum::{
    Router,
    routing::{get, post},
};
use config::{AppState, ServerConfig};
use minijinja::Environment;
use std::sync::Arc;
use tower_http::services::ServeDir;

async fn graceful_shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler");
}

#[tokio::main]
async fn main() {
    let config_path =
        std::env::var("CONFIG_PATH").unwrap_or_else(|_| "/etc/dashing/config.json".to_string());

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let mut environment = Environment::new();
    minijinja_embed::load_templates!(&mut environment);
    let assets =
        ServeDir::new(std::env::var("ASSETS_PATH").unwrap_or_else(|_| "/assets".to_string()));

    let config = match ServerConfig::load(&config_path) {
        Ok(cfg) => cfg,
        Err(err) => panic!(
            "Could not open the configuration file in path {:?}: {:?}",
            config_path, err
        ),
    };

    let app = Router::new()
        .route("/", get(routes::root))
        .route("/utils", get(routes::icons))
        .route("/links", get(routes::links))
        .route("/settings", get(routes::settings))
        .route("/save-settings", post(routes::save_settings))
        .nest_service("/assets", assets)
        .with_state(Arc::new(AppState::new(environment, config, &config_path)));

    let listener = tokio::net::TcpListener::bind("[::]:8080").await.unwrap();
    tracing::info!("Listening on [::]:8080");

    axum::serve(listener, app)
        .with_graceful_shutdown(graceful_shutdown_signal())
        .await
        .unwrap();
}
