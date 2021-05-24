use crate::domain::model::identity::Id;
use crate::UserResp;
use anyhow::Result;
use thiserror::Error;
use warp::reject::Reject;

pub type UserId = Id<User>;

#[derive(Debug)]
pub struct User {
    user_id: UserId,
    name: UserName,
}

#[derive(Debug)]
pub struct UserName(String);

impl UserName {
    pub fn new(s: &str) -> Result<Self> {
        if s.len() >= 100 {
            return Err(UserError::InvalidUserName)?;
        }
        Ok(UserName(s.to_owned()))
    }

    pub fn string(self) -> String {
        self.0
    }
}

impl User {
    pub fn new(user_id: UserId, name: UserName) -> User {
        User { user_id, name }
    }

    // 主にmapping用途
    pub fn propeties(self) -> (Id<User>, UserName) {
        (self.user_id, self.name)
    }

    // これはだめだが...
    pub fn to_resp(self) -> UserResp {
        UserResp {
            id: self.user_id.string(),
            name: self.name.string(),
        }
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
