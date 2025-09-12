use crate::model::Locale;
use axum::body::Bytes;
use axum_typed_multipart::TryFromMultipart;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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
}

#[typeshare]
#[derive(Deserialize, Debug, TryFromMultipart)]
#[serde(rename_all = "camelCase")]
#[try_from_multipart(rename_all = "camelCase")]
pub struct InteractionInput {
    pub locale: Locale,
    #[serde(skip)]
    pub user_agent: Option<String>,
    #[serde(skip)]
    pub file: Bytes,
}

impl InteractionInput {
    pub async fn insert_to_db(self, pool: &PgPool) -> sqlx::Result<Interaction> {
        sqlx::query_as("INSERT INTO interaction (locale, user_agent) VALUES ($1, $2) RETURNING *")
            .bind(self.locale)
            .bind(self.user_agent)
            .fetch_one(pool)
            .await
    }
}

impl Interaction {
    pub async fn get_by_id(id: Uuid, pool: &PgPool) -> sqlx::Result<Self> {
        sqlx::query_as("SELECT * FROM interaction WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
    }
}
