use crate::model::Locale;
use axum::body::Bytes;
use axum_typed_multipart::TryFromMultipart;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{PgPool, prelude::FromRow};
use typeshare::typeshare;
use uuid::Uuid;

#[typeshare]
#[derive(Serialize, Debug, FromRow, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Interaction {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub locale: Locale,
    pub user_agent: Option<String>,
    pub duration: i32,
}

#[typeshare]
#[derive(Debug, TryFromMultipart, Serialize)]
#[serde(rename_all = "camelCase")]
#[try_from_multipart(rename_all = "camelCase")]
pub struct InteractionInput {
    pub locale: Locale,
    pub duration: i32,
    #[serde(skip)] // comment this line when generating types with typeshare
    pub file: Bytes,
}
