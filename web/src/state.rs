use crate::config::ServerConfig;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::broadcast;

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: PgPool,
    pub server_config: ServerConfig,
    pub ws_sender: broadcast::Sender<String>,
    pub mutable_data: Arc<Mutex<Option<i32>>>,
}

impl AppState {
    pub fn new(pool: PgPool, config: ServerConfig) -> Self {
        Self {
            pool,
            server_config: config,
            ws_sender: broadcast::channel(100).0,
            mutable_data: Arc::new(Mutex::new(None)),
        }
    }
}
