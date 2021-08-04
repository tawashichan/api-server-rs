use crate::{
    domain::{
        model::{
            email::Email,
            error::DomainError,
            login::LoginPassword,
            user::{UserId, UserName},
        },
        service::{
            login_service::{ILoginService, LoginRequest},
            user_service::{self, IUserService},
        },
    },
    init::Services,
    presenter::{self, user_response::UserResponse},
};
use axum::{
    extract::{Extension, Json, UrlParams},
    prelude::*,
    response::IntoResponse,
};
use serde_json::json;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct HealthCheckResp {
    pub result: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    status_code: u16,
    message: String,
    error_type: String,
}

pub async fn find_user_handler(
    UrlParams((id,)): UrlParams<(String,)>,
    Extension(services): Extension<Arc<Services>>,
) -> Result<response::Json<UserResponse>, DomainError> {
    let user_id = UserId::new_from_string(&id)?;
    match services.user_service.find_by_id(&user_id).await {
        Ok(user) => Ok(response::Json(
            presenter::user_response::UserResponse::from_model(user),
        )),
        Err(err) => Err(err),
    }
}

#[derive(Deserialize)]
pub struct CreateUserReq {
    name: String,
    email: String,
}

pub async fn create_user_handler(
    Extension(services): Extension<Arc<Services>>,
    Json(req): Json<CreateUserReq>,
) -> Result<impl IntoResponse, DomainError> {
    let name = UserName::new(&req.name)?;
    let email = Email::new(&req.email)?;

    let service_req = user_service::CreateUserReq::new(name, email);
    match services.user_service.create_user(service_req).await {
        Ok(()) => Ok(response::Json(json!({
            "result": "ok"
        }))),
        Err(err) => Err(err),
    }
}
/*
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
*/
