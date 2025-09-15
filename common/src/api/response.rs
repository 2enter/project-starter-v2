use crate::api::error::ApiError;
use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ApiResult<T>
where
    T: Serialize,
{
    Ok(T),
    Err(ApiError),
}

/// Convert Result into ApiResponse
impl<T: Serialize, K: Display> From<Result<T, K>> for ApiResult<T> {
    fn from(value: Result<T, K>) -> Self {
        match value {
            Ok(value) => ApiResult::Ok(value),
            Err(error) => ApiResult::Err(
                ApiError::from(StatusCode::INTERNAL_SERVER_ERROR).with_details(format!("{error}")),
            ),
        }
    }
}

impl<T> ApiResult<T>
where
    T: Serialize,
{
    pub fn new_success(data: T) -> Self {
        Self::Ok(data)
    }

    pub fn new_error<K>(code: K) -> Self
    where
        ApiError: From<K>,
    {
        Self::Err(ApiError::from(code))
    }

    /// Build the response into json value:
    /// ```json
    /// // When Err
    /// {
    ///     "data": null,
    ///     "error": ApiError
    /// }
    /// // When Ok
    /// {
    ///     "data": T,
    ///     "error": null
    /// }
    /// ```
    pub fn build(self) -> Json<Value> {
        match self {
            Self::Ok(data) => Json(json!({"data": data, "error": null})),
            Self::Err(error) => Json(json!({"data": null, "error": error})),
        }
    }
}

impl<T> IntoResponse for ApiResult<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        self.build().into_response()
    }
}
