use crate::domain::evolution::GrowthStage;
use crate::domain::{EvolutionEvent, GameState, Timestamp};

pub const EXPEDITION_DURATION_SECONDS: Timestamp = 60 * 60;
const EXPEDITION_ENERGY_COST: u8 = 10;
const EXPEDITION_EXPERIENCE_REWARD: u32 = 5;
const EXPEDITION_MOOD_REWARD: u8 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Expedition {
    pub expedition_type: ExpeditionType,
    pub started_at: Timestamp,
    pub returns_at: Timestamp,
    pub seed: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpeditionType {
    Explore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpeditionError {
    Locked,
    NotHatched,
    AlreadyAway,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpeditionOutcome {
    pub expedition_type: ExpeditionType,
    pub started_at: Timestamp,
    pub returns_at: Timestamp,
    pub evolution: Option<EvolutionEvent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpeditionResult {
    pub expedition_type: ExpeditionType,
    pub experience_gained: u32,
    pub mood_delta: i32,
}

impl ExpeditionType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Explore => "Explore",
        }
    }
}

impl GameState {
    pub fn start_expedition(
        &mut self,
        timestamp: Timestamp,
        day: Timestamp,
        seed: u64,
    ) -> Result<ExpeditionOutcome, ExpeditionError> {
        if self.pet.stage == GrowthStage::Egg {
            return Err(ExpeditionError::NotHatched);
        }

        if self.pet.stage == GrowthStage::Baby {
            return Err(ExpeditionError::Locked);
        }

        if self.expedition.is_some() {
            return Err(ExpeditionError::AlreadyAway);
        }

        self.daily_report.reset_if_new_day(day);
        self.pet.status.energy = self
            .pet
            .status
            .energy
            .saturating_sub(EXPEDITION_ENERGY_COST);

        let expedition = Expedition {
            expedition_type: ExpeditionType::Explore,
            started_at: timestamp,
            returns_at: timestamp.saturating_add(EXPEDITION_DURATION_SECONDS),
            seed,
        };
        self.expedition = Some(expedition);
        self.daily_report.record_expedition_started(timestamp);

        Ok(ExpeditionOutcome {
            expedition_type: expedition.expedition_type,
            started_at: expedition.started_at,
            returns_at: expedition.returns_at,
            evolution: None,
        })
    }

    pub fn complete_expedition_if_due(&mut self, timestamp: Timestamp) -> Option<ExpeditionResult> {
        let expedition = self.expedition?;
        if timestamp < expedition.returns_at {
            return None;
        }

        self.expedition = None;
        self.pet.experience = self
            .pet
            .experience
            .saturating_add(EXPEDITION_EXPERIENCE_REWARD);
        self.pet.status.mood = self
            .pet
            .status
            .mood
            .saturating_add(EXPEDITION_MOOD_REWARD)
            .min(100);
        self.queue_growth();
        self.daily_report.record_expedition_completed(
            timestamp,
            EXPEDITION_EXPERIENCE_REWARD,
            i32::from(EXPEDITION_MOOD_REWARD),
        );

        Some(ExpeditionResult {
            expedition_type: expedition.expedition_type,
            experience_gained: EXPEDITION_EXPERIENCE_REWARD,
            mood_delta: i32::from(EXPEDITION_MOOD_REWARD),
        })
    }
}
