use std::convert::Infallible;

use serde::Serialize;
use warp::{hyper::StatusCode, Rejection, Reply};

use crate::domain::model::error::DomainError;

#[derive(Serialize)]
pub struct ErrorResponse {
    status_code: u16,
    message: String,
    error_type: String,
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {

    let (status_code,message,error_type) = match err.find::<DomainError>() {
        Some(DomainError::DBError(s)) => {
            (500,"error!!!!".into(), "internal_server_error".into())
        }
        Some(DomainError::UserNotFound) => {
            (404,"user_not_found".into(), "user_not_found".into())
        }
        _ => {
            dbg!(err);
            (500,"error!!!!".into(), "internal_server_error".into())
        }
    };

    let json = warp::reply::json(&ErrorResponse {
        status_code,
        message,
        error_type,
    });

    Ok(warp::reply::with_status(
        json,
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}
