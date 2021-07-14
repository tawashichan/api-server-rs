use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::error::DomainError;

pub struct LoginPassword(String);

impl LoginPassword {
    pub fn new(s: &str) -> Result<LoginPassword, DomainError> {
        Ok(LoginPassword(s.to_string()))
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginToken(pub String);
