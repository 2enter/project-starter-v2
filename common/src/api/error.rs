use crate::api::response::ApiResponse;
use axum::Json;
use axum::response::{IntoResponse, Response};
use axum_typed_multipart::TypedMultipartError;
use http::StatusCode;
use regex::Regex;
use serde::{Deserialize, Serialize};

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

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = StatusCode::from_u16(self.code).unwrap();
        Json(ApiResponse::<String>::new_error_with_details(
            status_code,
            &self.message,
            None,
        ))
        .into_response()
    }
}

impl From<TypedMultipartError> for ApiError {
    fn from(err: TypedMultipartError) -> Self {
        ApiError::new_with_details(StatusCode::BAD_REQUEST, err.to_string(), None)
    }
}
