use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub database_config: DatabaseConfig,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub valid_permissions: Vec<String>,
}