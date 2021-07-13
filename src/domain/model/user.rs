use crate::domain::model::{email::Email, identity::Id};
use anyhow::Result;
use thiserror::Error;
use warp::reject::Reject;

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
    pub fn new(s: &str) -> Result<Self, UserError> {
        if s.len() >= 100 {
            return Err(UserError::InvalidUserName);
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

#[derive(Error, Debug)]
pub enum UserError {
    #[error("user_not_found")]
    NotFound,
    #[error("invalid_user_name")]
    InvalidUserName,
}

impl Reject for UserError {}
