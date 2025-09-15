use crate::api::response::ApiResult;
use axum::response::{IntoResponse, Response};
use axum_typed_multipart::TypedMultipartError;
use http::StatusCode;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
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

impl From<u16> for ApiError {
    fn from(code: u16) -> Self {
        let code = StatusCode::from_u16(code).unwrap();
        Self::from(code)
    }
}

impl From<StatusCode> for ApiError {
    fn from(code: StatusCode) -> Self {
        Self {
            code: code.as_u16(),
            message: gen_message(code),
            ..Default::default()
        }
    }
}

impl ApiError {
    pub fn with_details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }

    pub fn with_hint(mut self, hint: String) -> Self {
        self.hint = Some(hint);
        self
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        ApiResult::<String>::Err(self).into_response()
    }
}

impl From<TypedMultipartError> for ApiError {
    fn from(err: TypedMultipartError) -> Self {
        ApiError::from(StatusCode::BAD_REQUEST).with_details(err.to_string())
    }
}
