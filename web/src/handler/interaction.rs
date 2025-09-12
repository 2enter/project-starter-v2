use crate::state::AppState;
use axum::{
    extract::{Json, Path, State},
    http::{HeaderMap, header},
};
use axum_typed_multipart::BaseMultipart;
use common::{
    api::{ApiError, ApiResponse},
    model::{Interaction, InteractionInput},
};
use uuid::Uuid;

pub async fn post(
    State(app_state): State<AppState>,
    header_map: HeaderMap,
    BaseMultipart {
        data: interaction, ..
    }: BaseMultipart<InteractionInput, ApiError>,
) -> Json<ApiResponse<Interaction>> {
    let user_agent = if let Some(user_agent) = header_map.get(header::USER_AGENT) {
        user_agent.to_str().ok().map(str::to_owned)
    } else {
        None
    };

    let result: ApiResponse<_> = sqlx::query_as(
        "INSERT INTO interaction (locale, user_agent, duration) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(interaction.locale)
    .bind(user_agent)
    .bind(interaction.duration)
    .fetch_one(&app_state.pool)
    .await
    .into();

    result.into()
}

pub async fn get_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<ApiResponse<Interaction>> {
    let result: ApiResponse<_> = sqlx::query_as("SELECT * FROM interaction WHERE id = $1")
        .bind(id)
        .fetch_one(&app_state.pool)
        .await
        .into();
    result.into()
}
