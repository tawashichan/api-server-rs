use crate::domain::model::error::DomainError;
use crate::domain::model::{email::Email, identity::Id, user::User, user::UserName};
use crate::domain::traits::{id_generator::IIdGenerator, user_repository::IUserRepository};
use crate::infra::{id_generator::IdGenerator, user_repository::UserRepository};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct CreateUserReq {
    name: UserName,
    email: Email,
}

impl CreateUserReq {
    pub fn new(name: UserName, email: Email) -> Self {
        CreateUserReq { name, email }
    }
}

#[async_trait]
pub trait IUserService {
    type UserRepository: IUserRepository + Sync + Send;
    type IdGenerator: IIdGenerator + Sync + Send;

    fn new(
        user_repository: Arc<Self::UserRepository>,
        id_generator: Arc<Self::IdGenerator>,
    ) -> Self;

    fn user_repository(&self) -> &Self::UserRepository;

    fn id_generator(&self) -> &Self::IdGenerator;

    async fn find_by_id(&self, user_id: &Id<User>) -> Result<User, DomainError> {
        self.user_repository().find_by_id(user_id).await
    }

    async fn create_user(&self, req: CreateUserReq) -> Result<(), DomainError> {
        let id = self.id_generator().generate::<User>();
        let user = User::new(id, req.name, req.email);
        self.user_repository().save(user).await
    }
}

pub struct UserService {
    user_repository: Arc<UserRepository>,
    id_generator: Arc<IdGenerator>,
}

impl IUserService for UserService {
    type UserRepository = UserRepository;
    type IdGenerator = IdGenerator;

    fn new(
        user_repository: Arc<Self::UserRepository>,
        id_generator: Arc<Self::IdGenerator>,
    ) -> Self {
        UserService {
            user_repository,
            id_generator,
        }
    }
    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
    fn id_generator(&self) -> &Self::IdGenerator {
        &self.id_generator
    }
}
