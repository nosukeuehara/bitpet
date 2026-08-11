use crate::application::result::ApplicationResult;
use crate::application::ApplicationError;
use crate::domain::action::{Action, ActionError, ActionOutcome};
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
        let state = self.load_update_and_save()?;
        Ok(state)
    }

    pub fn feed(&mut self) -> ApplicationResult<ActionOutcome> {
        self.perform_action(Action::Feed)
    }

    pub fn play(&mut self) -> ApplicationResult<ActionOutcome> {
        self.perform_action(Action::Play)
    }

    fn perform_action(&mut self, action: Action) -> ApplicationResult<ActionOutcome> {
        let now = self.clock.now();
        let mut state = self.load_and_update_time(now)?;
        let day = time::day_index(now);

        match action {
            Action::Feed => state.feed(day),
            Action::Play => state.play(day),
            Action::Go => Ok(()),
        }
        .map_err(application_action_error)?;

        self.repository.save(&state)?;
        Ok(ActionOutcome { action, state })
    }

    fn load_update_and_save(&mut self) -> ApplicationResult<GameState> {
        let now = self.clock.now();
        let state = self.load_and_update_time(now)?;
        self.repository.save(&state)?;
        Ok(state)
    }

    fn load_and_update_time(&mut self, now: u64) -> ApplicationResult<GameState> {
        let mut state = if self.repository.exists() {
            self.repository.load()?
        } else {
            GameState::new(now)
        };
        let loaded_version = state.version;

        if loaded_version < 2 {
            state.version = SAVE_VERSION;
            state.last_updated_at = now;
        } else {
            time::apply_elapsed_time(&mut state, now);
        }

        let day = time::day_index(now);
        if loaded_version < SAVE_VERSION {
            state.version = SAVE_VERSION;
            state.daily_actions = crate::domain::DailyActions::new(day);
            if loaded_version < 4 {
                state.care_stats = crate::domain::CareStats::new();
                state.pet.update_growth(state.care_stats);
            }
        } else {
            state.daily_actions.reset_if_new_day(day);
        }

        Ok(state)
    }
}

fn application_action_error(error: ActionError) -> ApplicationError {
    match error {
        ActionError::DailyLimitReached(action) => ApplicationError::ActionLimitReached(action),
    }
}
