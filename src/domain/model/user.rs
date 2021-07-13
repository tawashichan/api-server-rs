use crate::domain::model::{email::Email, identity::Id};
use anyhow::Result;

use super::error::DomainError;

pub type UserId = Id<User>;

#[derive(Debug)]
pub struct User {
    user_id: UserId,
    name: UserName,
    email: Email,
}

#[derive(Debug)]
pub struct UserName(String);

impl UserName {
    pub fn new(s: &str) -> Result<Self, DomainError> {
        if s.len() >= 100 {
            return Err(DomainError::InvalidUserName);
        }
        Ok(UserName(s.to_owned()))
    }

    pub fn string(self) -> String {
        self.0
    }
}

impl User {
    pub fn new(user_id: UserId, name: UserName, email: Email) -> User {
        User {
            user_id,
            name,
            email,
        }
    }

    // 主にmapping用途
    pub fn propeties(self) -> (Id<User>, UserName, Email) {
        (self.user_id, self.name, self.email)
    }
}
