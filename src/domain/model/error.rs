use thiserror::Error;
use warp::reject::Reject;

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

impl Reject for DomainError {}
