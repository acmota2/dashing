use minijinja::Environment;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{File, write};
use tokio::sync::RwLock;

pub struct AppState {
    pub file_path: String,
    pub environment: Environment<'static>,
    pub config: RwLock<ServerConfig>,
}

impl AppState {
    pub fn new(environment: Environment<'static>, config: ServerConfig, config_path: &str) -> Self {
        Self {
            file_path: config_path.to_string(),
            environment,
            config: RwLock::new(config),
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
    pub fn load(config_path: &str) -> std::io::Result<Self> {
        Ok(serde_json::from_reader(File::open(config_path)?)?)
    }

    pub fn write_to_file(&self, config_path: &str) -> std::io::Result<()> {
        let data = serde_json::to_string(self)?;
        write(config_path, data)
    }

    pub fn rewrite_config(json: &str) -> Result<Self, serde_json::Error> {
        Ok(serde_json::from_str(json)?)
    }
}
