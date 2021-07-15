use crate::error_handler::handle_rejection;
use crate::handler::{create_user_handler, find_user_handler, login_handler, HealthCheckResp};
use init::Services;
use warp::hyper::body::Bytes;
use std::convert::Infallible;
use std::sync::Arc;
use tokio;
use warp::{self, Buf, Rejection};
use warp::Filter;

pub mod config;
pub mod domain;
pub mod error_handler;
pub mod handler;
pub mod infra;
pub mod init;
pub mod presenter;

fn with_services(
    services: Arc<Services>,
) -> impl Filter<Extract = (Arc<Services>,), Error = Infallible> + Clone {
    warp::any().map(move || services.clone())
}

fn log_body() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    warp::body::bytes()
        .map(|b: Bytes| {
            println!(
                "Request body: {}",
                std::str::from_utf8(b.bytes()).expect("error converting bytes to &str")
            );
        })
        .untuple_one()
}

#[tokio::main]
async fn main() {
    let conf = match config::Config::init() {
        Ok(conf) => conf,
        Err(e) => panic!("{:?}", e),
    };

    let services = init::init(&conf);

    let routing_base = warp::any().and(with_services(services));

    let health_check = warp::path!("health_check").map(|| {
        warp::reply::json(&HealthCheckResp {
            result: "ok".to_string(),
        })
    });

    let health_check_root = warp::path::end().map(|| {
        warp::reply::json(&HealthCheckResp {
            result: "ok".to_string(),
        })
    });

    let user_path = warp::path("users");
    let login_path = warp::path("login");

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

    let login = routing_base
        .clone()
        .and(login_path)
        .and(warp::post())
        .and(warp::body::json())
        .and_then(login_handler);

    let routing = warp::any()
        .and(health_check_root)
        .or(health_check)
        .or(create_user)
        .or(find_user)
        .or(login)
        .recover(handle_rejection);

    println!("start server");

    warp::serve(routing).run(([0, 0, 0, 0], 8080)).await;
}
