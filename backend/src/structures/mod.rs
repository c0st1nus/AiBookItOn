use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct DbConfig {
    pub host: String,
    pub port: String,
    pub name: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Config {
    pub host: String,
    pub port: String,
    pub db: DbConfig,
}