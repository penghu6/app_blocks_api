use std::env;
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub database_url: String,
    pub app_port: u16,
    pub upload_dir: String,
}

impl Settings {
    fn new() -> Self {
        Self {
            database_url: get_env("DATABASE_URL"),
            app_port: get_env_or("APP_PORT", "8080").parse().unwrap(),
            upload_dir: get_env_or("UPLOAD_DIR", "uploads"),
        }
    }
}

fn get_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("{} must be set", key))
}

fn get_env_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new();
}

