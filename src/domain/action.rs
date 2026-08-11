use crate::domain::{GameState, Timestamp};

pub const DAILY_ACTION_LIMIT: u32 = 3;
const FEED_HUNGER_GAIN: u8 = 20;
const FEED_MOOD_GAIN: u8 = 5;
const PLAY_MOOD_GAIN: u8 = 10;
const PLAY_ENERGY_COST: u8 = 10;
const PLAY_EXPERIENCE_GAIN: u32 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Feed,
    Play,
    Go,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionError {
    DailyLimitReached(Action),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionOutcome {
    pub action: Action,
    pub state: GameState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DailyActions {
    pub day: Timestamp,
    pub feed_count: u32,
    pub play_count: u32,
}

impl DailyActions {
    pub const fn new(day: Timestamp) -> Self {
        Self {
            day,
            feed_count: 0,
            play_count: 0,
        }
    }

    pub fn reset_if_new_day(&mut self, day: Timestamp) {
        if self.day != day {
            *self = Self::new(day);
        }
    }
}

impl GameState {
    pub fn feed(&mut self, day: Timestamp) -> Result<(), ActionError> {
        self.daily_actions.reset_if_new_day(day);
        if self.daily_actions.feed_count >= DAILY_ACTION_LIMIT {
            return Err(ActionError::DailyLimitReached(Action::Feed));
        }

        self.pet.status.hunger = self
            .pet
            .status
            .hunger
            .saturating_add(FEED_HUNGER_GAIN)
            .min(100);
        self.pet.status.mood = self.pet.status.mood.saturating_add(FEED_MOOD_GAIN).min(100);
        self.daily_actions.feed_count += 1;
        Ok(())
    }

    pub fn play(&mut self, day: Timestamp) -> Result<(), ActionError> {
        self.daily_actions.reset_if_new_day(day);
        if self.daily_actions.play_count >= DAILY_ACTION_LIMIT {
            return Err(ActionError::DailyLimitReached(Action::Play));
        }

        self.pet.status.mood = self.pet.status.mood.saturating_add(PLAY_MOOD_GAIN).min(100);
        self.pet.status.energy = self.pet.status.energy.saturating_sub(PLAY_ENERGY_COST);
        self.pet.experience = self.pet.experience.saturating_add(PLAY_EXPERIENCE_GAIN);
        self.daily_actions.play_count += 1;
        Ok(())
    }
}
