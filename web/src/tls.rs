use axum_server::tls_rustls::RustlsConfig;
use std::path::PathBuf;

use crate::config::ServerConfig;

pub fn init() {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("no crypto provider");
}

pub async fn get_axum_config(config: &ServerConfig) -> RustlsConfig {
    let root_dir = PathBuf::from(&config.root_dir).join("cert");
    RustlsConfig::from_pem_file(root_dir.join("cert.pem"), root_dir.join("key.pem"))
        .await
        .unwrap()
}
