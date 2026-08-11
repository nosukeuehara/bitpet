use crate::application::result::ApplicationResult;
use crate::domain::GameState;
use crate::infrastructure::storage::GameRepository;

pub struct GameService<R> {
    repository: R,
}

impl<R> GameService<R>
where
    R: GameRepository,
{
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn status(&mut self) -> ApplicationResult<GameState> {
        if self.repository.exists() {
            self.repository.load()
        } else {
            let state = GameState::default();
            self.repository.save(&state)?;
            Ok(state)
        }
    }
}
