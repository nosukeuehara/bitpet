pub mod action;
pub mod evolution;
pub mod expedition;
pub mod pet;
pub mod report;
pub mod status;

pub use pet::Pet;

pub const SAVE_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub version: u32,
    pub pet: Pet,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            version: SAVE_VERSION,
            pet: Pet::default(),
        }
    }
}
