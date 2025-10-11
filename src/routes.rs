use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{Form, Html, IntoResponse},
};
use minijinja::context;
use serde::Deserialize;
use serde_json::{json, to_string_pretty};

use crate::config::{AppState, ServerConfig};

pub async fn root(State(state): State<Arc<AppState>>) -> Html<String> {
    let index = state.environment.get_template("index.html").unwrap();
    let config = state.config.read().await;
    Html(
        index
            .render(context! { server_name => config.server_name, is_home_route => true })
            .unwrap(),
    )
}

pub async fn icons(State(state): State<Arc<AppState>>) -> Html<String> {
    let icons = state.environment.get_template("util-section.html").unwrap();
    let config = state.config.read().await;
    Html(
        icons
            .render(context! { sections => config.sections })
            .unwrap(),
    )
}

pub async fn links(State(state): State<Arc<AppState>>) -> Html<String> {
    let links = state.environment.get_template("links.html").unwrap();
    let config = state.config.read().await;
    Html(links.render(context! { links => config.links }).unwrap())
}

pub async fn settings(State(state): State<Arc<AppState>>) -> Html<String> {
    let settings = state.environment.get_template("index.html").unwrap();
    let config = state.config.read().await;
    let config_json = json!(*config);
    Html(
        settings
            .render(context! { json_content => to_string_pretty(&config_json).unwrap(), is_home_route => false, server_name => config.server_name })
            .unwrap(),
    )
}

#[derive(Deserialize)]
pub struct SaveSettings {
    json_content: String,
}

pub async fn save_settings(
    State(state): State<Arc<AppState>>,
    Form(payload): Form<SaveSettings>,
) -> impl IntoResponse {
    let template = state
        .environment
        .get_template("save-settings-notification.html")
        .unwrap();
    let new_config = match ServerConfig::rewrite_config(&payload.json_content) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("[ERROR] Wrong JSON format: {}", e);
            let html = template
                .render(context! { message => "Could not save settings" })
                .unwrap();
            return (StatusCode::OK, Html(html)).into_response();
        }
    };

    if let Err(e) = new_config.write_to_file(None) {
        eprintln!("[ERROR] Failed to write config to file: {}", e);
        let html = template
            .render(context! { message => "Could not write configuration to file" })
            .unwrap();
        return (StatusCode::OK, Html(html)).into_response();
    }
    println!("[INFO] Wrote file successfully");

    let mut config_guard = state.config.write().await;
    *config_guard = new_config;
    let html = template
        .render(context! { message => "Settings saved successfully" })
        .unwrap();
    (StatusCode::OK, Html(html)).into_response()
}
