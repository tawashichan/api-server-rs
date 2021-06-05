use crate::domain::model::{login::LoginToken, user::User};
use anyhow::Result;

pub trait IJWTHandler {
    fn generate(&self, user: &User) -> Result<LoginToken> {
        unimplemented!()
    }
}
