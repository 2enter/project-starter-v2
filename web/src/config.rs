use common::utils::{get_env, get_root_dir};

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
        let port = get_env("BACKEND_PORT");
        let host = get_env("BACKEND_HOST");
        let root_dir = get_root_dir();

        println!("configuration initialized: {host}:{port}, db: {database_url},  root: {root_dir}");

        Self {
            https: false,
            database_url,
            port: port.parse().unwrap_or(3000),
            host,
            root_dir,
        }
    }
}
