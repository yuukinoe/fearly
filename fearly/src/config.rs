

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub routers: RoutersConfig,
    pub database: DatabaseConfig,
    pub directories: DirectoriesConfig,
    pub debug: DebugConfig,
    pub session_duration: i64,
}


#[derive(Debug, Serialize, Deserialize)]

pub struct DebugConfig {
    pub prints: bool,
    pub cookies: bool,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoriesConfig {
    pub media: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RoutersConfig {
    pub api: Vec<Router>,
    pub web: Vec<Router>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Router {
    pub protocol: String,
    pub hostname: String,
    pub port: u16,
    pub pathname: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
}
