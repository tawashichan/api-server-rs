use crate::domain::model::{identity::Id, user::User};
use crate::infra::user_repository::UserRepository;
use anyhow::Result;
use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;

#[async_trait]
pub trait IUserRepository {
    async fn find_by_id(&self, user_id: &Id<User>) -> Result<User>;
}

#[async_trait]
pub trait IUserService {
    type UserRepository: IUserRepository + Sync + Send;

    fn new(user_repository: Arc<Self::UserRepository>) -> Self;

    fn user_repository(&self) -> &Self::UserRepository;

    async fn find_by_id(&self, user_id: &Id<User>) -> Result<User> {
        self.user_repository().find_by_id(user_id).await
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
