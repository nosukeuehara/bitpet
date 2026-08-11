pub mod action;
pub mod evolution;
pub mod expedition;
pub mod hatching;
pub mod monster;
pub mod pet;
pub mod report;
pub mod status;
pub mod time;

pub use action::{CareStats, DailyActions};
pub use evolution::{EvolutionEvent, PendingEvolution};
pub use expedition::Expedition;
pub use hatching::HatchingState;
pub use monster::{MonsterFamily, SpeciesId};
pub use pet::Pet;
pub use report::{DailyReport, LoginState};

pub type Timestamp = u64;

pub const SAVE_VERSION: u32 = 9;

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
    pub hatching: Option<HatchingState>,
    pub pending_evolution: Option<PendingEvolution>,
}

impl GameState {
    pub fn new(last_updated_at: Timestamp) -> Self {
        let day = time::day_index(last_updated_at);
        Self::new_with_day(last_updated_at, day)
    }

    pub fn new_with_day(last_updated_at: Timestamp, day: Timestamp) -> Self {
        Self {
            version: SAVE_VERSION,
            pet: Pet::default(),
            last_updated_at,
            daily_actions: DailyActions::new(day),
            care_stats: CareStats::new(),
            daily_report: DailyReport::new(day),
            login: LoginState::new(),
            expedition: None,
            hatching: Some(HatchingState::new(last_updated_at)),
            pending_evolution: None,
        }
    }

    pub fn apply_growth(&mut self) -> Option<EvolutionEvent> {
        if self.pending_evolution.is_some() {
            return None;
        }

        self.pet.update_growth(self.care_stats)
    }

    pub fn queue_growth(&mut self) -> Option<PendingEvolution> {
        if self.pending_evolution.is_some() {
            return self.pending_evolution;
        }

        let event = self.pet.evolution_candidate(self.care_stats)?;
        let pending = PendingEvolution::from(event);
        self.pending_evolution = Some(pending);
        Some(pending)
    }

    pub fn resolve_pending_evolution(&mut self) -> Option<EvolutionEvent> {
        let pending = self.pending_evolution.take()?;
        let event = EvolutionEvent::from(pending);
        self.pet.apply_evolution(event);
        Some(event)
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new(0)
    }
}
