use std::sync::Arc;

use axum::{Form, extract::State, response::Html};
use minijinja::context;
use serde::{Deserialize, Serialize};

use crate::config::{AppState, Link, ServerConfig, UtilSection};

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
            .render(context! { is_home_route => false, server_name => state.config.server_name, sections => state.config.sections })
            .unwrap(),
    )
}

pub async fn settings_section(State(state): State<Arc<AppState>>) -> Html<String> {
    let settings_sections = state
        .environment
        .get_template("settings-section.html")
        .unwrap();
    Html(
        settings_sections
            .render(context! {
            section => UtilSection {
                name: "".to_owned(),
                utils: vec![],
            }})
            .unwrap(),
    )
}

pub async fn settings_link(State(state): State<Arc<AppState>>) -> Html<String> {
    let settings_links = state
        .environment
        .get_template("settings-link.html")
        .unwrap();
    Html(
        settings_links
            .render(
                context! { link => Link { name: "".to_owned(), link: "".to_owned(), icon: None }},
            )
            .unwrap(),
    )
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FormItems {
    pub items: Vec<UtilSection>,
}

pub async fn settings_post(Form(new_sections): Form<FormItems>) -> () {
    // Html<String> {
    println!("{:?}", new_sections);
}
