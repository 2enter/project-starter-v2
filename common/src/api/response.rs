use crate::api::error::ApiError;
use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug)]
pub enum ApiResponse<T>
where
    T: Serialize,
{
    Success(T),
    Error(ApiError),
}

impl<T: Serialize, K: Display> From<Result<T, K>> for ApiResponse<T> {
    fn from(value: Result<T, K>) -> Self {
        match value {
            Ok(value) => ApiResponse::Success(value),
            Err(error) => ApiResponse::Error(
                ApiError::from(StatusCode::INTERNAL_SERVER_ERROR).with_details(format!("{error}")),
            ),
        }
    }
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn new_success(data: T) -> Self {
        Self::Success(data)
    }

    pub fn new_error<K>(code: K) -> Self
    where
        ApiError: From<K>,
    {
        Self::Error(ApiError::from(code))
    }

    pub fn build(self) -> Json<Value> {
        match self {
            Self::Success(data) => Json(json!({"data": data, "error": null})),
            Self::Error(error) => Json(json!({"data": null, "error": error})),
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        self.build().into_response()
    }
}
