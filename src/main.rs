use crate::handler::{create_user_handler, find_user_handler, HealthCheckResp};
use domain::model::error::DomainError;
use std::net::SocketAddr;
use tokio;

use axum::prelude::*;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;

pub mod config;
pub mod domain;
pub mod handler;
pub mod infra;
pub mod init;
pub mod presenter;

#[tokio::main]
async fn main() {
    let conf = match config::Config::init() {
        Ok(conf) => conf,
        Err(e) => panic!("{:?}", e),
    };

    let services = init::init(&conf);

    /*let routing_base = warp::any().and(with_services(services));

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

    warp::serve(routing).run(([0, 0, 0, 0], 8080)).await;*/

    let app = route("/health_check", get(health_check))
        .route("/users/:id", get(find_user_handler))
        .route("/users", post(create_user_handler))
        .layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(services))
                .into_inner(),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_check() -> Result<response::Json<HealthCheckResp>, DomainError> {
    Ok(response::Json(HealthCheckResp {
        result: "ok".to_string(),
    }))
}
