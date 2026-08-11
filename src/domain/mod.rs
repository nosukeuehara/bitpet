pub mod action;
pub mod evolution;
pub mod expedition;
pub mod pet;
pub mod report;
pub mod status;
pub mod time;

pub use action::{CareStats, DailyActions};
pub use expedition::Expedition;
pub use pet::Pet;
pub use report::{DailyReport, LoginState};

pub type Timestamp = u64;

pub const SAVE_VERSION: u32 = 6;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub version: u32,
    pub pet: Pet,
    pub last_updated_at: Timestamp,
    pub daily_actions: DailyActions,
    pub care_stats: CareStats,
    pub daily_report: DailyReport,
    pub login: LoginState,
    pub expedition: Option<Expedition>,
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
            daily_report: DailyReport::new(day),
            login: LoginState::new(),
            expedition: None,
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new(0)
    }
}
