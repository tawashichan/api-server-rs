use crate::domain::model::{identity::Id, user::User, user::UserId, user::UserName};
use crate::domain::repositories::user_repository::IUserRepository;
use crate::infra::user_repository::UserRepository;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait IUserService {
    type UserRepository: IUserRepository + Sync + Send;

    fn new(user_repository: Arc<Self::UserRepository>) -> Self;

    fn user_repository(&self) -> &Self::UserRepository;

    async fn find_by_id(&self, user_id: &Id<User>) -> Result<User> {
        self.user_repository().find_by_id(user_id).await
    }

    async fn create_user(&self, name: &str) -> Result<()> {
        let id = UserId::new();
        let name = UserName::new(name)?;
        
        let user = User::new(id, name);
        self.user_repository().save(user).await
    }
}

pub struct UserService {
    user_repository: Arc<UserRepository>,
}

impl IUserService for UserService {
    type UserRepository = UserRepository;
    fn new(user_repository: Arc<Self::UserRepository>) -> Self {
        UserService { user_repository }
    }
    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
}
