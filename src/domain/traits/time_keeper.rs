pub trait ITimekeeper {
    fn now(&self) -> i64;
}
