use crate::domain::{GameState, Timestamp};

pub const HATCH_DURATION_SECONDS: Timestamp = 60 * 60;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HatchingState {
    pub egg_created_at: Timestamp,
    pub hatches_at: Timestamp,
}

impl HatchingState {
    pub fn new(egg_created_at: Timestamp) -> Self {
        Self {
            egg_created_at,
            hatches_at: egg_created_at.saturating_add(HATCH_DURATION_SECONDS),
        }
    }
}

impl GameState {
    pub fn hatch_if_due(&mut self, now: Timestamp) -> bool {
        let Some(hatching) = self.hatching else {
            return false;
        };

        if now < hatching.hatches_at {
            return false;
        }

        self.pet.hatch();
        self.hatching = None;
        true
    }
}
