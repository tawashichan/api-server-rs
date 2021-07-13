use crate::domain::model::{user::UserError, user::UserId};
use crate::domain::service::user_service::IUserService;
use anyhow::Result;
use domain::model::email::Email;
use domain::model::user::UserName;
use domain::service::user_service;
use init::Services;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::Arc;
use tokio;
use warp;
use warp::http::StatusCode;
use warp::reject::Rejection;
use warp::reply::Reply;
use warp::{reject, Filter};

pub mod config;
pub mod domain;
pub mod infra;
pub mod init;
pub mod libs;
pub mod presenter;
pub mod web;

async fn find_user_handler(
    services: Arc<Services>,
    id: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let user_id = UserId::new_from_string(&id).unwrap();
    match services.user_service.find_by_id(&user_id).await {
        Ok(user) => Ok(warp::reply::json(
            &presenter::user_response::UserResponse::from_model(user),
        )),
        Err(err) => Err(warp::reject::custom(UserError::NotFound)),
    }
}

#[derive(Deserialize)]
struct CreateUserReq {
    name: String,
    email: String,
}

async fn create_user_handler(
    services: Arc<Services>,
    req: CreateUserReq,
) -> Result<impl warp::Reply, warp::Rejection> {
    let name = UserName::new(&req.name).map_err(|e| reject::custom(e))?;
    let email = Email::new(&req.email).map_err(|e| reject::custom(e))?;

    let service_req = user_service::CreateUserReq::new(name, email);
    match services.user_service.create_user(service_req).await {
        Ok(()) => Ok(StatusCode::OK),
        Err(err) => Err(warp::reject::custom(UserError::NotFound)),
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    status_code: u16,
    message: String,
    error_type: String,
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let json = warp::reply::json(&ErrorResponse {
        status_code: 500,
        message: "error!!!!".into(),
        error_type: "aaa".into(),
    });

    Ok(warp::reply::with_status(
        json,
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

fn with_services(
    services: Arc<Services>,
) -> impl Filter<Extract = (Arc<Services>,), Error = Infallible> + Clone {
    warp::any().map(move || services.clone())
}

#[tokio::main]
async fn main() {
    let conf = match config::Config::init() {
        Ok(conf) => conf,
        Err(e) => panic!("{:?}", e),
    };

    let services = init::init(&conf);

    let routing_base = warp::any().and(with_services(services));

    let health_check = warp::path!("health_check").map(|| StatusCode::OK);

    let user_path = warp::path("users");

    let find_user = routing_base
        .clone()
        .and(user_path)
        .and(warp::get())
        .and(warp::path!(String))
        .and_then(find_user_handler);

    let create_user = routing_base
        .clone()
        .and(user_path)
        .and(warp::post())
        .and(warp::body::json())
        .and_then(create_user_handler);

    let find_user = health_check
        .or(find_user)
        .or(create_user)
        .recover(handle_rejection);

    warp::serve(find_user).run(([127, 0, 0, 1], 8888)).await;
}
