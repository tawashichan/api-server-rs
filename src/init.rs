use crate::domain::service::user_service::{IUserService, UserService};
use crate::infra::user_repository::UserRepository;
use std::sync::Arc;
use rusoto_dynamodb::{DynamoDbClient};
use rusoto_core;

pub struct Infra {
    pub user_repository: Arc<UserRepository>,
}

pub struct Services {
    pub user_service: UserService,
}

fn init_infra() -> Infra {
    let dynamodb_client = Arc::new(DynamoDbClient::new(rusoto_core::region::Region::ApNortheast1));
    let user_repository = UserRepository::new(dynamodb_client);

    Infra {
        user_repository: Arc::new(user_repository),
    }
}

fn init_services(infra: Infra) -> Services {
    let user_service = UserService::new(infra.user_repository.clone());
    Services { user_service }
}

pub fn init() -> Arc<Services> {
    let infra = init_infra();
    Arc::new(init_services(infra))
}
