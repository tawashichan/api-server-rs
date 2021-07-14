use crate::domain::model::{error::DomainError, login::LoginToken, user::User};
use anyhow::Result;

use super::time_keeper::ITimekeeper;

pub trait IJWTHandler {
    type Timekeeper: ITimekeeper;

    fn time_keeper(&self) -> &Self::Timekeeper;

    fn generate(&self, user: &User) -> Result<LoginToken, DomainError>;
}
