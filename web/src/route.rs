use crate::handler::*;
use crate::state::AppState;
use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::routing::{any, get, post};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

pub fn get_route(app_state: AppState) -> Router {
    Router::new()
        .route("/ws", any(ws::handler))
        .nest(
            "/api",
            Router::new()
                .route("/health", get(health::get))
                .layer(DefaultBodyLimit::max(5000000))
                .nest(
                    "/soul",
                    Router::new()
                        .route("/{id}", get(interaction::get_by_id))
                        .route("/", post(interaction::post)),
                )
                .nest_service(
                    "/storage",
                    ServeDir::new(format!(
                        "{}/db/storage/public",
                        app_state.server_config.root_dir
                    )),
                ),
        )
        .layer(TraceLayer::new_for_http())
        .fallback_service(ServeDir::new(format!(
            "{}/svelte/build",
            app_state.server_config.root_dir
        )))
        .with_state(app_state)
}
