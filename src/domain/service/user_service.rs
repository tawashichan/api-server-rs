use crate::domain::model::{identity::Id, user::User, user::UserId, user::UserName};
use crate::domain::traits::{id_generator::IIdGenerator, user_repository::IUserRepository};
use crate::infra::{id_generator::IdGenerator, user_repository::UserRepository};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

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

    async fn find_by_id(&self, user_id: &Id<User>) -> Result<User> {
        self.user_repository().find_by_id(user_id).await
    }

    async fn create_user(&self, name: &str) -> Result<()> {
        let id = self.id_generator().generate::<User>();
        let name = UserName::new(name)?;
        let user = User::new(id, name);
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
