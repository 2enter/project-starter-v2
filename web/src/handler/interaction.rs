use crate::state::AppState;
use axum::{
    extract::{Json, State},
    http::{HeaderMap, header},
};
use axum_typed_multipart::BaseMultipart;
use common::{
    api::{ApiError, ApiResponse},
    model::{Interaction, InteractionInput},
};

#[axum::debug_handler]
pub async fn post(
    State(app_state): State<AppState>,
    header_map: HeaderMap,
    BaseMultipart {
        data: mut interaction,
        ..
    }: BaseMultipart<InteractionInput, ApiError>,
) -> Json<ApiResponse<Interaction>> {
    let user_agent = if let Some(user_agent) = header_map.get(header::USER_AGENT) {
        user_agent.to_str().ok().map(str::to_owned)
    } else {
        None
    };

    interaction.user_agent = user_agent;
    let result = interaction.insert_to_db(&app_state.pool).await;

    let res: ApiResponse<_> = result.into();
    res.into()
}

pub async fn get_by_id() {}
