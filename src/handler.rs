use std::sync::Arc;

use warp::{hyper::StatusCode, reject};

use crate::{
    domain::{
        model::{
            email::Email,
            login::LoginPassword,
            user::{UserId, UserName},
        },
        service::{
            login_service::{ILoginService, LoginRequest},
            user_service::{self, IUserService},
        },
    },
    init::Services,
    presenter,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct HealthCheckResp {
    pub result: String,
}

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
        Err(err) => Err(warp::reject::custom(err)),
    }
}

#[derive(Deserialize)]
pub struct LoginRawRequest {
    email: String,
    password: String,
}

pub async fn login_handler(
    services: Arc<Services>,
    req: LoginRawRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let email = Email::new(&req.email).map_err(|e| reject::custom(e))?;
    let password = LoginPassword::new(&req.password).map_err(|e| reject::custom(e))?;

    let service_req = LoginRequest { email, password };
    match services.login_service.login(service_req).await {
        Ok(resp) => Ok(warp::reply::json(&resp)),
        Err(err) => Err(warp::reject::custom(err)),
    }
}
