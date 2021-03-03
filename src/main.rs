use crate::domain::model::identity::Id;
use crate::domain::service::user_service::IUserService;
use init::Services;
use serde::Serialize;
use std::convert::Infallible;
use std::sync::Arc;
use tokio;
use warp;
use warp::Filter;

pub mod domain;
pub mod infra;
pub mod init;

#[derive(Serialize)]
pub struct UserResp {
    pub id: String,
    pub name: String,
}

async fn find_user(services: Arc<Services>, id: String) -> Result<impl warp::Reply,warp::Rejection> {
    let user_id = Id::new(id);
    match services.user_service.find_by_id(&user_id).await {
        Ok(user) => Ok(warp::reply::json(&user.to_resp())),
        Err(err) => {
            println!("{:?}",err);
            Err(warp::reject())
        },
    }
}

fn with_services(
    services: Arc<Services>,
) -> impl Filter<Extract = (Arc<Services>,), Error = Infallible> + Clone {
    warp::any().map(move || services.clone())
}

#[tokio::main]
async fn main() {
    let services = init::init();

    let find_user = warp::get()
        .and(warp::path("users"))
        .and(with_services(services))
        .and(warp::path::param::<String>())
        .and_then(find_user);

    warp::serve(find_user).run(([127, 0, 0, 1], 8888)).await;
}
