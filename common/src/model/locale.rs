use axum_typed_multipart::TryFromField;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Deserialize, sqlx::Type, Debug, Clone, TryFromField)]
#[serde(rename_all = "kebab-case")]
#[sqlx(type_name = "locale", rename_all = "kebab-case")]
pub enum Locale {
    En,
    ZhTw,
}
