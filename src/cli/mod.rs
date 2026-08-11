pub mod commands;
pub mod renderer;

use crate::application::ApplicationError;
use crate::application::GameService;
use crate::infrastructure::storage::FileRepository;
use commands::Command;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub enum CliError {
    UnknownCommand(String),
}

impl Display for CliError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownCommand(command) => write!(formatter, "Unknown command: {command}"),
        }
    }
}

impl Error for CliError {}

pub fn run<I>(args: I) -> Result<(), Box<dyn Error>>
where
    I: IntoIterator<Item = String>,
{
    let command = Command::parse(args)?;
    let output = match command {
        Command::Help(topic) => renderer::render_help(topic),
        Command::Version => env!("CARGO_PKG_VERSION").to_string(),
        Command::Update { check_only } => run_update_command(check_only)?,
        command => run_game_command(command)?,
    };

    println!("{output}");
    Ok(())
}

fn run_game_command(command: Command) -> Result<String, Box<dyn Error>> {
    let repository = FileRepository::from_default_save_dir()?;
    let mut service = GameService::new(repository);
    let output = match command {
        Command::Status => renderer::render_status_outcome(&service.status()?),
        Command::Feed => match service.feed() {
            Ok(outcome) => renderer::render_action_outcome(&outcome),
            Err(ApplicationError::ActionLimitReached(action)) => {
                renderer::render_action_limit_reached(action)
            }
            Err(ApplicationError::PetAway) => renderer::render_pet_away(),
            Err(ApplicationError::PetNotHatched) => renderer::render_pet_not_hatched(),
            Err(error) => return Err(error.into()),
        },
        Command::Play => match service.play() {
            Ok(outcome) => renderer::render_action_outcome(&outcome),
            Err(ApplicationError::ActionLimitReached(action)) => {
                renderer::render_action_limit_reached(action)
            }
            Err(ApplicationError::PetAway) => renderer::render_pet_away(),
            Err(ApplicationError::PetNotHatched) => renderer::render_pet_not_hatched(),
            Err(error) => return Err(error.into()),
        },
        Command::Go => match service.start_expedition() {
            Ok(outcome) => renderer::render_expedition_started(&outcome),
            Err(ApplicationError::ExpeditionLocked) => renderer::render_expedition_locked(),
            Err(ApplicationError::PetNotHatched) => renderer::render_pet_not_hatched(),
            Err(ApplicationError::PetAway) => renderer::render_pet_away(),
            Err(error) => return Err(error.into()),
        },
        Command::Report => renderer::render_report(&service.report()?),
        Command::Streak => renderer::render_streak(&service.streak()?),
        Command::Update { .. } | Command::Help(_) | Command::Version => {
            unreachable!("handled before repository access")
        }
    };

    Ok(output)
}

#[cfg(feature = "self_update")]
fn run_update_command(check_only: bool) -> Result<String, Box<dyn Error>> {
    use crate::infrastructure::self_update::{self, UpdateOutcome};

    let outcome = if check_only {
        self_update::check_for_updates(env!("CARGO_PKG_VERSION"))?
    } else {
        self_update::update(env!("CARGO_PKG_VERSION"))?
    };

    Ok(match outcome {
        UpdateOutcome::UpToDate { current } => renderer::render_update_up_to_date(&current),
        UpdateOutcome::Available { current, latest } => {
            renderer::render_update_available(&current, &latest)
        }
        UpdateOutcome::Updated { previous, current } => {
            renderer::render_update_success(&previous, &current)
        }
    })
}

#[cfg(not(feature = "self_update"))]
fn run_update_command(_check_only: bool) -> Result<String, Box<dyn Error>> {
    Ok("Self-update is available only in native CLI builds.".to_string())
}
