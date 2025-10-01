use std::sync::Arc;

use axum::{extract::State, response::Html};
use minijinja::context;

use crate::config::AppState;

pub async fn root(State(state): State<Arc<AppState>>) -> Html<String> {
    let index = state.environment.get_template("index.html").unwrap();
    Html(
        index
            .render(context! { server_name => state.config.server_name, is_home_route => true })
            .unwrap(),
    )
}

pub async fn icons(State(state): State<Arc<AppState>>) -> Html<String> {
    let icons = state.environment.get_template("util-section.html").unwrap();
    Html(
        icons
            .render(context! { sections => state.config.sections })
            .unwrap(),
    )
}

pub async fn links(State(state): State<Arc<AppState>>) -> Html<String> {
    let links = state.environment.get_template("links.html").unwrap();
    Html(
        links
            .render(context! { links => state.config.links })
            .unwrap(),
    )
}

pub async fn settings(State(state): State<Arc<AppState>>) -> Html<String> {
    let settings = state.environment.get_template("index.html").unwrap();
    Html(
        settings
            .render(context! { json_content => state.config, is_home_route => false, server_name => state.config.server_name })
            .unwrap(),
    )
}
