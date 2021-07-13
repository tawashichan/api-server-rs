use std::sync::Arc;

use warp::{hyper::StatusCode, reject};

use crate::{
    domain::{
        model::{
            email::Email,
            error::DomainError,
            user::{UserId, UserName},
        },
        service::user_service::{self, IUserService},
    },
    init::Services,
    presenter,
};

use serde::Deserialize;

pub async fn find_user_handler(
    services: Arc<Services>,
    id: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let user_id = UserId::new_from_string(&id).map_err(|e| reject::custom(e))?;
    match services.user_service.find_by_id(&user_id).await {
        Ok(user) => Ok(warp::reply::json(
            &presenter::user_response::UserResponse::from_model(user),
        )),
        Err(err) => Err(warp::reject::custom(err)),
    }
}

#[derive(Deserialize)]
pub struct CreateUserReq {
    name: String,
    email: String,
}

pub async fn create_user_handler(
    services: Arc<Services>,
    req: CreateUserReq,
) -> Result<impl warp::Reply, warp::Rejection> {
    let name = UserName::new(&req.name).map_err(|e| reject::custom(e))?;
    let email = Email::new(&req.email).map_err(|e| reject::custom(e))?;

    let service_req = user_service::CreateUserReq::new(name, email);
    match services.user_service.create_user(service_req).await {
        Ok(()) => Ok(StatusCode::OK),
        Err(err) => Err(warp::reject::custom(DomainError::UserNotFound)),
    }
}
