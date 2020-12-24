#[derive(Debug)]
pub struct Id<IdType>(String, std::marker::PhantomData<IdType>);

impl<T> Id<T> {
    pub fn new(id: String) -> Self {
        Id(id, std::marker::PhantomData)
    }

    pub fn string(&self) -> String {
        self.0.clone()
    }
}
