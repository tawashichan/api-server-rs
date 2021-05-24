use crate::domain::model::{user::UserError, user::UserId};
use crate::domain::service::user_service::IUserService;
use init::Services;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::Arc;
use tokio;
use warp;
use warp::http::StatusCode;
use warp::reject::Rejection;
use warp::reply::Reply;
use warp::Filter;

pub mod domain;
pub mod infra;
pub mod init;
pub mod libs;

#[derive(Serialize)]
pub struct UserResp {
    pub id: String,
    pub name: String,
}

async fn find_user_handler(
    services: Arc<Services>,
    id: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let user_id = UserId::new_from_string(&id).unwrap();
    match services.user_service.find_by_id(&user_id).await {
        Ok(user) => Ok(warp::reply::json(&user.to_resp())),
        Err(err) => Err(warp::reject::custom(UserError::NotFound)),
    }
}

#[derive(Deserialize)]
struct CreateUserReq {
    name: String,
}

async fn create_user_handler(
    services: Arc<Services>,
    id: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let user_id = UserId::new_from_string(&id).unwrap();
    match services.user_service.find_by_id(&user_id).await {
        Ok(user) => Ok(warp::reply::json(&user.to_resp())),
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
    let services = init::init();

    /*let find_user = warp::any()
    .and(
        warp::get().and(
            warp::path("users")
                .and(warp::path!(String))
                .and_then(move |id| find_user_handler(services.clone(), id)),
        ),
    )
    .recover(handle_rejection);*/

    let find_user = warp::any()
        .and(
            warp::path("users").and(
                warp::get()
                    .and(with_services(services))
                    .and(warp::path!(String))
                    .and_then(find_user_handler),
            ), /*.or(warp::post()
               .and_then(move || create_user_handler(services.clone(), "aaa".into()))),*/
        )
        .recover(handle_rejection);

    warp::serve(find_user).run(([127, 0, 0, 1], 8888)).await;
}
