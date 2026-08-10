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

    pub fn status(&self) -> ApplicationResult<GameState> {
        self.repository.load()
    }
}
