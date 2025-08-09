use minijinja::Environment;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    collections::HashMap,
    fs::{self, File},
    iter::zip,
};

pub struct AppState {
    pub environment: Environment<'static>,
    pub config: ServerConfig,
    pub form_state: HashMap<(String, usize), usize>,
}

impl AppState {
    pub fn new(environment: Environment<'static>, config: ServerConfig) -> Self {
        let form_state = zip(
            config.sections.iter(),
            (0..config.sections.len()).into_iter(),
        )
        .map(|(v, i)| ((v.name.clone(), i), v.utils.len()))
        .collect();

        Self {
            environment,
            config,
            form_state,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServerConfig {
    pub server_name: String,
    pub links: Vec<Link>,
    pub sections: Vec<UtilSection>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Link {
    pub name: String,
    pub link: String,
    pub icon: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UtilSection {
    pub name: String,
    pub utils: Vec<Link>,
}

impl ServerConfig {
    pub fn new() -> Self {
        Self {
            server_name: "".to_owned(),
            links: Vec::new(),
            sections: Vec::new(),
        }
    }

    pub fn populate(&mut self, path: Option<&str>) -> std::io::Result<Self> {
        Ok(serde_json::from_reader(File::open(
            path.unwrap_or("./config.json"),
        )?)?)
    }

    pub fn write_to(&self, path: Option<&str>) -> std::io::Result<()> {
        let data = serde_json::to_string(self)?;
        fs::write(path.unwrap_or("./config.json"), data)
    }
}
