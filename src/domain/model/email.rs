use anyhow::Result;

use super::user::UserError;

#[derive(Debug, Clone)]
pub struct Email(String);

impl Email {
    pub fn new(s: &str) -> Result<Email, UserError> {
        Ok(Email(s.to_owned()))
    }

    pub fn string(&self) -> String {
        self.0.clone()
    }
}
