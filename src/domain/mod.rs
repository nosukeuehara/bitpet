pub mod action;
pub mod evolution;
pub mod expedition;
pub mod pet;
pub mod report;
pub mod status;

pub use pet::Pet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub version: u32,
    pub pet: Pet,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            version: 1,
            pet: Pet::default(),
        }
    }
}
