use crate::api::error::ApiError;
use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    data: Option<T>,
    error: Option<ApiError>,
}

impl<T: Serialize, K: Display> From<Result<T, K>> for ApiResponse<T> {
    fn from(value: Result<T, K>) -> Self {
        match value {
            Ok(value) => ApiResponse::new_success(value),
            Err(error) => ApiResponse::new_error_with_details(
                StatusCode::NOT_FOUND,
                &format!("{error}"),
                None,
            ),
        }
    }
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn new_success(data: T) -> Self {
        Self {
            data: Some(data),
            error: None,
        }
    }

    pub fn new_error(code: StatusCode) -> Self {
        Self {
            data: None,
            error: Some(ApiError::new(code)),
        }
    }

    pub fn new_error_with_details(code: StatusCode, details: &str, hint: Option<String>) -> Self {
        Self {
            data: None,
            error: Some(ApiError::new_with_details(code, details.to_string(), hint)),
        }
    }

    pub fn new_not_found(details: &str) -> Self {
        Self {
            data: None,
            error: Some(ApiError::new_with_details(
                StatusCode::NOT_FOUND,
                details.to_string(),
                None,
            )),
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
