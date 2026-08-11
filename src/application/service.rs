use crate::application::result::ApplicationResult;
use crate::application::ApplicationError;
use crate::domain::action::{Action, ActionError, ActionOutcome};
use crate::domain::expedition::{ExpeditionError, ExpeditionOutcome};
use crate::domain::{time, DailyReport, EvolutionEvent, GameState, LoginState, SAVE_VERSION};
use crate::infrastructure::clock::{Clock, SystemClock};
use crate::infrastructure::storage::GameRepository;

pub struct GameService<R, C = SystemClock> {
    repository: R,
    clock: C,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusOutcome {
    pub state: GameState,
    pub evolution: Option<EvolutionEvent>,
}

impl std::ops::Deref for StatusOutcome {
    type Target = GameState;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
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

    pub fn into_repository(self) -> R {
        self.repository
    }

    pub fn status(&mut self) -> ApplicationResult<StatusOutcome> {
        self.load_update_and_save(true)
    }

    pub fn feed(&mut self) -> ApplicationResult<ActionOutcome> {
        self.perform_action(Action::Feed)
    }

    pub fn play(&mut self) -> ApplicationResult<ActionOutcome> {
        self.perform_action(Action::Play)
    }

    pub fn start_expedition(&mut self) -> ApplicationResult<ExpeditionOutcome> {
        let now = self.clock.now();
        let day = self.clock.day(now);
        let (mut state, evolution) = self.load_and_update_time(now, true)?;
        let mut outcome = state
            .start_expedition(now, day, now)
            .map_err(application_expedition_error)?;
        outcome.evolution = evolution;

        self.repository.save(&state)?;
        Ok(outcome)
    }

    pub fn report(&mut self) -> ApplicationResult<DailyReport> {
        let state = self.load_update_and_save(false)?.state;
        Ok(state.daily_report)
    }

    pub fn streak(&mut self) -> ApplicationResult<LoginState> {
        let state = self.load_update_and_save(false)?.state;
        Ok(state.login)
    }

    fn perform_action(&mut self, action: Action) -> ApplicationResult<ActionOutcome> {
        let now = self.clock.now();
        let day = self.clock.day(now);
        let (mut state, pending_evolution) = self.load_and_update_time(now, true)?;

        let action_evolution = match action {
            Action::Feed => state.feed(now, day),
            Action::Play => state.play(now, day),
            Action::Go => return Err(ApplicationError::InvalidAction),
        }
        .map_err(application_action_error)?;

        let evolution = action_evolution.or(pending_evolution);
        self.repository.save(&state)?;
        Ok(ActionOutcome {
            action,
            state,
            evolution,
        })
    }

    fn load_update_and_save(
        &mut self,
        resolve_evolution: bool,
    ) -> ApplicationResult<StatusOutcome> {
        let now = self.clock.now();
        let (state, evolution) = self.load_and_update_time(now, resolve_evolution)?;
        self.repository.save(&state)?;
        Ok(StatusOutcome { state, evolution })
    }

    fn load_and_update_time(
        &mut self,
        now: u64,
        resolve_evolution: bool,
    ) -> ApplicationResult<(GameState, Option<EvolutionEvent>)> {
        let day = self.clock.day(now);
        let mut state = if self.repository.exists() {
            self.repository.load()?
        } else {
            GameState::new_with_day(now, day)
        };
        let loaded_version = state.version;

        if loaded_version < 2 {
            state.version = SAVE_VERSION;
            state.last_updated_at = now;
        } else if state.pet.is_egg() {
            if let Some(hatching) = state.hatching {
                if now >= hatching.hatches_at {
                    state.last_updated_at = hatching.hatches_at;
                    state.hatch_if_due(now);
                    time::apply_elapsed_time(&mut state, now);
                } else {
                    time::apply_elapsed_time(&mut state, now);
                }
            } else {
                return Err(ApplicationError::InvalidSaveData);
            }
        } else {
            time::apply_elapsed_time(&mut state, now);
        }

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

        if loaded_version < 8 {
            state.hatching = None;
        }

        if loaded_version < 9 {
            state.pending_evolution = None;
        }

        if loaded_version < SAVE_VERSION {
            state.version = SAVE_VERSION;
        }
        state.record_login(day, now);
        state.complete_expedition_if_due(now);
        let evolution = if resolve_evolution {
            state.resolve_pending_evolution()
        } else {
            None
        };

        Ok((state, evolution))
    }
}

fn application_action_error(error: ActionError) -> ApplicationError {
    match error {
        ActionError::DailyLimitReached(action) => ApplicationError::ActionLimitReached(action),
        ActionError::NotHatched => ApplicationError::PetNotHatched,
        ActionError::PetAway => ApplicationError::PetAway,
    }
}

fn application_expedition_error(error: ExpeditionError) -> ApplicationError {
    match error {
        ExpeditionError::Locked => ApplicationError::ExpeditionLocked,
        ExpeditionError::NotHatched => ApplicationError::PetNotHatched,
        ExpeditionError::AlreadyAway => ApplicationError::PetAway,
    }
}
