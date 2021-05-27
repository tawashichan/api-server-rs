use crate::domain::model::identity::Id;

pub trait IIdGenerator {
    fn generate<T>(&self) -> Id<T>;
}
