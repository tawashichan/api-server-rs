use crate::error_handler::handle_rejection;
use crate::handler::{create_user_handler, find_user_handler};
use init::Services;
use std::convert::Infallible;
use std::sync::Arc;
use tokio;
use warp;
use warp::http::StatusCode;
use warp::{Filter};

pub mod config;
pub mod domain;
pub mod error_handler;
pub mod handler;
pub mod infra;
pub mod init;
pub mod libs;
pub mod presenter;

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
