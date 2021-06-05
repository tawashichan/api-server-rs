use crate::domain::model::identity::Id;
use crate::domain::traits::id_generator::IIdGenerator;

pub struct IdGenerator {}

impl IdGenerator {
    pub fn new() -> Self {
        IdGenerator{}
    }
}

impl IIdGenerator for IdGenerator {
    fn generate<T>(&self) -> Id<T> {
        Id::new()
    }
}
