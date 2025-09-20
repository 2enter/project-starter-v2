mod config;
mod handler;
mod route;
mod state;
mod tls;

use common::utils::{get_pg_pool, init_tracing};
use config::ServerConfig;
use std::net::SocketAddr;

use crate::{route::get_route, state::AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _guard = init_tracing(
        vec![
            "web=debug",
            "common=debug",
            "tower_http=debug",
            "tokio=debug",
            "axum=debug",
            "sqlx=debug",
        ],
        true,
    );

    let server_config = ServerConfig::new();
    let pool = get_pg_pool().await;
    let app_state = AppState::new(pool, server_config.clone());
    let app = get_route(app_state);

    // 0.0.0.0:<BACKEND_PORT>
    let socket_addr = SocketAddr::from(([0; 4], server_config.port));

    match server_config.https {
        true => {
            tls::init();
            let tls_config = tls::get_axum_config(&server_config).await;
            axum_server::bind_rustls(socket_addr, tls_config)
                .serve(app.into_make_service())
                .await?;
        }
        false => {
            let listener = tokio::net::TcpListener::bind(socket_addr).await?;
            axum::serve(listener, app).await?;
        }
    }

    Ok(())
}
