use std::sync::Arc;

use crate::domain::traits::time_keeper::ITimekeeper;
use anyhow::Result;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::domain::{
    model::{error::DomainError, identity::Id, login::LoginToken, user::User},
    traits::jwt_handler::IJWTHandler,
};

use super::time_keeper::TimeKeeper;

pub struct JWTHandler {
    time_keeper: Arc<TimeKeeper>,
    private_key: Vec<u8>,
}

#[derive(Debug, Serialize)]
struct JWTClaim {
    exp: usize,
    user_id: Id<User>,
}

impl JWTHandler {
    pub fn new(time_keeper: Arc<TimeKeeper>) -> JWTHandler {
        JWTHandler {
            time_keeper,
            private_key: vec![],
        }
    }
}

impl IJWTHandler for JWTHandler {
    type Timekeeper = super::time_keeper::TimeKeeper;

    fn time_keeper(&self) -> &Self::Timekeeper {
        &self.time_keeper
    }

    fn generate(&self, user: &User) -> Result<LoginToken, DomainError> {
        let header = Header::new(jsonwebtoken::Algorithm::RS256);
        let claims = JWTClaim {
            exp: self.time_keeper().now() as usize,
            user_id: user.id(),
        };
        let token = encode(
            &header,
            &claims,
            &EncodingKey::from_secret(&self.private_key),
        )
        .map_err(|_| DomainError::JWTError)?;
        Ok(LoginToken(token))
    }
}
