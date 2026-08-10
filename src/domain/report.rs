#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DailyReport {
    pub feed_count: u32,
    pub play_count: u32,
    pub adventure_count: u32,
}

impl DailyReport {
    pub const fn empty() -> Self {
        Self {
            feed_count: 0,
            play_count: 0,
            adventure_count: 0,
        }
    }
}
