use crate::config::Config;
use crate::domain::service::login_service::{ILoginService, LoginService};
use crate::domain::service::user_service::{IUserService, UserService};
use crate::infra::jwt_handler::JWTHandler;
use crate::infra::time_keeper::TimeKeeper;
use crate::infra::{id_generator::IdGenerator, user_repository::UserRepository};
use dynamodb::Client;
use std::sync::Arc;

pub struct Infra {
    pub user_repository: Arc<UserRepository>,
    pub id_generator: Arc<IdGenerator>,
    pub time_keeper: Arc<TimeKeeper>,
    pub jwt_handler: Arc<JWTHandler>,
}

pub struct Services {
    pub user_service: UserService,
    pub login_service: LoginService,
}

fn init_infra(conf: &Config) -> Infra {
    let aws_conf = dynamodb::Config::builder()
        .region(dynamodb::Region::new("ap-northeast-1"))
        .build();

    let dynamodb_client = Arc::new(Client::from_conf(aws_conf));
    let user_repository = UserRepository::new(conf, dynamodb_client);
    let id_generator = IdGenerator::new();
    let time_keeper = Arc::new(TimeKeeper::new());
    let jwt_handler = JWTHandler::new(time_keeper.clone());

    Infra {
        user_repository: Arc::new(user_repository),
        id_generator: Arc::new(id_generator),
        time_keeper: time_keeper.clone(),
        jwt_handler: Arc::new(jwt_handler),
    }
}

fn init_services(infra: Infra) -> Services {
    let user_service = UserService::new(infra.user_repository.clone(), infra.id_generator.clone());
    let login_service = LoginService::new(infra.jwt_handler.clone(), infra.user_repository.clone());
    Services {
        user_service,
        login_service,
    }
}

pub fn init(conf: &Config) -> Arc<Services> {
    let infra = init_infra(conf);
    Arc::new(init_services(infra))
}
