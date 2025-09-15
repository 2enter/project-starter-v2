use crate::config::ServerConfig;
use common::model::WsMsg;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::{Mutex, broadcast};

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

    pub fn ws_broadcast(&self, msg: WsMsg) -> anyhow::Result<()> {
        let clients = self.ws_sender.receiver_count();
        tracing::info!("Broadcasting to {} clients", clients);
        if clients == 0 {
            return Err(anyhow::anyhow!("No clients to broadcast to"));
        }
        let message = serde_json::to_string(&msg)?;
        self.ws_sender.send(message)?;
        Ok(())
    }
}
