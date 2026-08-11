pub mod action;
pub mod evolution;
pub mod expedition;
pub mod pet;
pub mod report;
pub mod status;
pub mod time;

pub use action::{CareStats, DailyActions};
pub use pet::Pet;

pub type Timestamp = u64;

pub const SAVE_VERSION: u32 = 4;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub version: u32,
    pub pet: Pet,
    pub last_updated_at: Timestamp,
    pub daily_actions: DailyActions,
    pub care_stats: CareStats,
}

impl GameState {
    pub fn new(last_updated_at: Timestamp) -> Self {
        let day = time::day_index(last_updated_at);
        Self {
            version: SAVE_VERSION,
            pet: Pet::default(),
            last_updated_at,
            daily_actions: DailyActions::new(day),
            care_stats: CareStats::new(),
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new(0)
    }
}
