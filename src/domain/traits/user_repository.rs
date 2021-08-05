use crate::domain::model::{email::Email, error::DomainError, user::User, user::UserId};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait IUserRepository {
    async fn find_by_id(&self, user_id: &UserId) -> Result<User, DomainError>;
    async fn find_by_email(&self, email: &Email) -> Result<User, DomainError>;
    async fn create(&self, user: User) -> Result<(), DomainError>;
}
