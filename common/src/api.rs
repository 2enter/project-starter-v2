use axum::Json;
use axum::response::{IntoResponse, Response};
use axum_typed_multipart::TypedMultipartError;
use http::StatusCode;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Debug, Deserialize)]
pub struct IdOnly {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    data: Option<T>,
    error: Option<ApiError>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
    pub details: Option<String>,
    pub hint: Option<String>,
}

fn gen_message(code: StatusCode) -> String {
    Regex::new(r"\d+")
        .unwrap()
        .replace_all(code.to_string().to_uppercase().as_str(), "")
        .trim()
        .replace(" ", "_")
        .to_string()
}

impl<T: Serialize, K: Display> From<Result<T, K>> for ApiResponse<T> {
    fn from(value: Result<T, K>) -> Self {
        match value {
            Ok(value) => ApiResponse::new_success(value),
            Err(error) => ApiResponse::new_error_with_details(
                StatusCode::NOT_FOUND,
                "",
                Some(format!("{error}")),
            ),
        }
    }
}

impl ApiError {
    pub fn new(code: StatusCode) -> Self {
        Self {
            code: code.as_u16(),
            message: gen_message(code),
            details: None,
            hint: None,
        }
    }

    pub fn from_u16(code_number: u16) -> Self {
        let code = StatusCode::from_u16(code_number).unwrap();
        Self::new(code)
    }

    pub fn new_with_details(code: StatusCode, details: String, hint: Option<String>) -> Self {
        Self {
            code: code.as_u16(),
            message: gen_message(code),
            details: Some(details),
            hint,
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

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = StatusCode::from_u16(self.code).unwrap();
        Json(ApiResponse::<String>::new_error(status_code)).into_response()
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

impl From<TypedMultipartError> for ApiError {
    fn from(err: TypedMultipartError) -> Self {
        ApiError::new_with_details(StatusCode::BAD_REQUEST, err.to_string(), None)
    }
}
