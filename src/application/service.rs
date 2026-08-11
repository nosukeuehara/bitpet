use crate::application::result::ApplicationResult;
use crate::domain::{time, GameState, SAVE_VERSION};
use crate::infrastructure::clock::{Clock, SystemClock};
use crate::infrastructure::storage::GameRepository;

pub struct GameService<R, C = SystemClock> {
    repository: R,
    clock: C,
}

impl<R> GameService<R, SystemClock>
where
    R: GameRepository,
{
    pub fn new(repository: R) -> Self {
        Self::with_clock(repository, SystemClock)
    }
}

impl<R, C> GameService<R, C>
where
    R: GameRepository,
    C: Clock,
{
    pub const fn with_clock(repository: R, clock: C) -> Self {
        Self { repository, clock }
    }

    pub fn status(&mut self) -> ApplicationResult<GameState> {
        let now = self.clock.now();
        let mut state = if self.repository.exists() {
            self.repository.load()?
        } else {
            GameState::new(now)
        };

        if state.version < SAVE_VERSION {
            state.version = SAVE_VERSION;
            state.last_updated_at = now;
        } else {
            time::apply_elapsed_time(&mut state, now);
        }

        self.repository.save(&state)?;
        Ok(state)
    }
}
