use axum::{prelude::*, response::IntoResponse};
use hyper::{Body, Response};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("user_not_found")]
    UserNotFound,
    #[error("invalid_user_name")]
    InvalidUserName,
    #[error("invalid_id_format")]
    InvalidIdFormat,
    #[error("db_error")]
    DBError(String),
    #[error("jwt_error")]
    JWTError,
}

impl IntoResponse for DomainError {
    fn into_response(self) -> Response<Body> {
        response::Json(json!({
            "aaa": "aaa"
        }))
        .into_response()
    }
}
