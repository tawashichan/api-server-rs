use crate::domain::model::{email::Email, user::User, user::UserId};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait IUserRepository {
    async fn find_by_id(&self, user_id: &UserId) -> Result<User>;
    async fn find_by_email(&self, email: &Email) -> Result<User>;
    async fn save(&self, user: User) -> Result<()>;
}
