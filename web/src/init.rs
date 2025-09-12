use tracing_subscriber::EnvFilter;

pub fn tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("web=debug".parse().unwrap())
                .add_directive("tower_http=debug".parse().unwrap())
                .add_directive("tokio=debug".parse().unwrap())
                .add_directive("axum=debug".parse().unwrap())
                .add_directive("sqlx=debug".parse().unwrap()),
        )
        .init();
}

pub mod tls {
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
}
