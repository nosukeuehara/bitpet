pub mod action;
pub mod evolution;
pub mod expedition;
pub mod pet;
pub mod report;
pub mod status;
pub mod time;

pub use pet::Pet;

pub type Timestamp = u64;

pub const SAVE_VERSION: u32 = 2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub version: u32,
    pub pet: Pet,
    pub last_updated_at: Timestamp,
}

impl GameState {
    pub fn new(last_updated_at: Timestamp) -> Self {
        Self {
            version: SAVE_VERSION,
            pet: Pet::default(),
            last_updated_at,
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new(0)
    }
}
