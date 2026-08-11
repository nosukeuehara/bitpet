use crate::application::result::ApplicationResult;
use crate::application::ApplicationError;
use crate::domain::action::{Action, ActionError, ActionOutcome};
use crate::domain::expedition::{ExpeditionError, ExpeditionOutcome};
use crate::domain::{time, DailyReport, GameState, LoginState, SAVE_VERSION};
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

    pub fn start_expedition(&mut self) -> ApplicationResult<ExpeditionOutcome> {
        let now = self.clock.now();
        let mut state = self.load_and_update_time(now)?;
        let outcome = state
            .start_expedition(now, now)
            .map_err(application_expedition_error)?;

        self.repository.save(&state)?;
        Ok(outcome)
    }

    pub fn report(&mut self) -> ApplicationResult<DailyReport> {
        let state = self.load_update_and_save()?;
        Ok(state.daily_report)
    }

    pub fn streak(&mut self) -> ApplicationResult<LoginState> {
        let state = self.load_update_and_save()?;
        Ok(state.login)
    }

    fn perform_action(&mut self, action: Action) -> ApplicationResult<ActionOutcome> {
        let now = self.clock.now();
        let mut state = self.load_and_update_time(now)?;

        match action {
            Action::Feed => state.feed(now),
            Action::Play => state.play(now),
            Action::Go => return Err(ApplicationError::InvalidAction),
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
        if loaded_version < 3 {
            state.daily_actions = crate::domain::DailyActions::new(day);
        } else {
            state.daily_actions.reset_if_new_day(day);
        }

        if loaded_version < 4 {
            state.care_stats = crate::domain::CareStats::new();
            state.pet.update_growth(state.care_stats);
        }

        if loaded_version < 5 {
            state.daily_report = DailyReport::new(day);
            state.login = LoginState::new();
        } else {
            state.daily_report.reset_if_new_day(day);
        }

        if loaded_version < 6 {
            state.expedition = None;
        }

        if loaded_version < SAVE_VERSION {
            state.version = SAVE_VERSION;
        }
        state.record_login(day, now);
        state.complete_expedition_if_due(now);

        Ok(state)
    }
}

fn application_action_error(error: ActionError) -> ApplicationError {
    match error {
        ActionError::DailyLimitReached(action) => ApplicationError::ActionLimitReached(action),
        ActionError::PetAway => ApplicationError::PetAway,
    }
}

fn application_expedition_error(error: ExpeditionError) -> ApplicationError {
    match error {
        ExpeditionError::Locked => ApplicationError::ExpeditionLocked,
        ExpeditionError::AlreadyAway => ApplicationError::PetAway,
    }
}
