use anyhow::Result;
use uuid;

use super::error::DomainError;

#[derive(Debug)]
pub struct Id<IdType>(uuid::Uuid, std::marker::PhantomData<IdType>);

impl<T> Id<T> {
    pub fn new() -> Self {
        Id(uuid::Uuid::new_v4(), std::marker::PhantomData)
    }

    pub fn new_from_string(s: &str) -> Result<Id<T>, DomainError> {
        Ok(Id(
            uuid::Uuid::parse_str(s).map_err(|e| DomainError::InvalidIdFormat)?,
            std::marker::PhantomData,
        ))
    }

    pub fn string(&self) -> String {
        self.0.to_string()
    }
}
