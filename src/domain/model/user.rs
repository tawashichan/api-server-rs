use crate::domain::model::identity::Id;
use crate::UserResp;
use thiserror::Error;

#[derive(Debug)]
pub struct User {
    user_id: Id<User>,
    name: String,
}

impl User {
    pub fn new(user_id: Id<User>, name: String) -> User {
        User { user_id, name }
    }

    // これはだめだが...
    pub fn to_resp(self) -> UserResp {
        UserResp {
            id: self.user_id.string(),
            name: self.name,
        }
    }
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error("user not found")]
    NotFound,
}
