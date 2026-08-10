use crate::application::{ApplicationError, ApplicationResult};
use crate::domain::GameState;

pub trait GameRepository {
    fn load(&self) -> ApplicationResult<GameState>;
}

#[derive(Debug, Default)]
pub struct MemoryRepository {
    state: GameState,
}

impl MemoryRepository {
    pub fn new(state: GameState) -> Self {
        Self { state }
    }
}

impl GameRepository for MemoryRepository {
    fn load(&self) -> ApplicationResult<GameState> {
        Ok(self.state.clone())
    }
}

pub fn storage_error(message: impl Into<String>) -> ApplicationError {
    ApplicationError::Storage(message.into())
}
