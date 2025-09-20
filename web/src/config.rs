use common::utils::{get_env, get_root_dir};
use tracing::info;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub database_url: String,
    pub port: u16,
    pub host: String,
    pub https: bool,
    pub root_dir: String,
}

#[allow(unused)]
impl ServerConfig {
    pub fn new() -> Self {
        let database_url = get_env("DATABASE_URL").replace("?sslmode=disable", "");
        let root_dir = get_root_dir();
        let port = get_env("BACKEND_PORT");
        // In many case we don't need this.
        // But for projects that hosts server across local network, it helps clients like Unreal Engine to locate the server easily.
        let host = get_env("BACKEND_HOST");
        let https = false;
        let http_scheme = if https { "https" } else { "http" };

        colored::control::set_override(true);
        info!("Configuration initialized");
        info!("Remote URL \t\t{:#}://{host}:{port}/", http_scheme);
        info!("Database URL \t{database_url}");
        info!("Root directory \t{root_dir}");

        Self {
            https,
            database_url,
            port: port.parse().unwrap_or(8080),
            host,
            root_dir,
        }
    }
}
