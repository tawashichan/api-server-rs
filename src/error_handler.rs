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
    let json = warp::reply::json(&ErrorResponse {
        status_code: 500,
        message: "error!!!!".into(),
        error_type: "internal_server_error".into(),
    });

    match err.find::<DomainError>() {
        Some(DomainError::DBError(s)) => {
            dbg!(s);
        }
        _ => {
            dbg!(err);
        }
    };

    Ok(warp::reply::with_status(
        json,
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}
