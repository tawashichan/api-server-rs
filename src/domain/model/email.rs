use anyhow::Result;

use super::error::DomainError;

#[derive(Debug, Clone)]
pub struct Email(String);

impl Email {
    pub fn new(s: &str) -> Result<Email, DomainError> {
        Ok(Email(s.to_owned()))
    }

    pub fn string(&self) -> String {
        self.0.clone()
    }
}
