use std::iter;
use std::sync::Arc;

use axum::{extract::State, response::Html};
use minijinja::{Environment, context};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Link {
    pub name: String,
    pub link: String,
    pub icon: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UtilSection {
    pub name: String,
    pub utils: Vec<Link>,
}

pub async fn root(State(env): State<Arc<Environment<'static>>>) -> Html<String> {
    let index = env.get_template("index.html").unwrap();
    Html(index.render(context! { server_name => "Blaze" }).unwrap())
}

pub async fn icons(State(env): State<Arc<Environment<'static>>>) -> Html<String> {
    let icons = env.get_template("util-section.html").unwrap();
    let link = iter::repeat(Link {
        name: "Proxmox".to_owned(),
        link: "https://pve.home.voldemota.xyz".to_owned(),
        icon: Some(
            "https://cdn.jsdelivr.net/gh/homarr-labs/dashboard-icons/svg/proxmox.svg".to_owned(),
        ),
    });
    let utils = iter::repeat(UtilSection {
        name: "Management".to_owned(),
        utils: link.take(30).collect(),
    });
    Html(
        icons
            .render(context! { sections => utils.take(3).collect::<Vec<UtilSection>>() })
            .unwrap(),
    )
}

pub async fn links(State(env): State<Arc<Environment<'static>>>) -> Html<String> {
    let links = env.get_template("links.html").unwrap();
    let links_collection = iter::repeat(Link {
        name: "YouTube".to_owned(),
        link: "https://youtube.com".to_owned(),
        icon: Some("/assets/fontawesome-youtube.svg".to_owned()),
    });
    Html(
        links
            .render(context! { links => links_collection.take(3).collect::<Vec<Link>>() })
            .unwrap(),
    )
}
