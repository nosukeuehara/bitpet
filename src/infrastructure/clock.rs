use crate::domain::Timestamp;
use std::time::{SystemTime, UNIX_EPOCH};

pub trait Clock {
    fn now(&self) -> Timestamp;
}

#[derive(Debug, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> Timestamp {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |duration| duration.as_secs())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FixedClock {
    now: Timestamp,
}

impl FixedClock {
    pub const fn new(now: Timestamp) -> Self {
        Self { now }
    }
}

impl Clock for FixedClock {
    fn now(&self) -> Timestamp {
        self.now
    }
}
