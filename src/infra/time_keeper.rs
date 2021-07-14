use crate::domain::traits::time_keeper::ITimekeeper;

pub struct TimeKeeper {}

impl ITimekeeper for TimeKeeper {
    fn now(&self) -> i64 {
        chrono::offset::Utc::now().timestamp()
    }
}
