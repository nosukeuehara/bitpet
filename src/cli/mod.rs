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
    let repository = FileRepository::from_default_save_dir()?;
    let mut service = GameService::new(repository);
    let output = match command {
        Command::Status => renderer::render_status(&service.status()?),
        Command::Feed => match service.feed() {
            Ok(outcome) => renderer::render_action_outcome(&outcome),
            Err(ApplicationError::ActionLimitReached(action)) => {
                renderer::render_action_limit_reached(action)
            }
            Err(ApplicationError::PetAway) => renderer::render_pet_away(),
            Err(error) => return Err(error.into()),
        },
        Command::Play => match service.play() {
            Ok(outcome) => renderer::render_action_outcome(&outcome),
            Err(ApplicationError::ActionLimitReached(action)) => {
                renderer::render_action_limit_reached(action)
            }
            Err(ApplicationError::PetAway) => renderer::render_pet_away(),
            Err(error) => return Err(error.into()),
        },
        Command::Go => match service.start_expedition() {
            Ok(outcome) => renderer::render_expedition_started(&outcome),
            Err(ApplicationError::ExpeditionLocked) => renderer::render_expedition_locked(),
            Err(ApplicationError::PetAway) => renderer::render_pet_away(),
            Err(error) => return Err(error.into()),
        },
        Command::Report => renderer::render_report(&service.report()?),
        Command::Streak => renderer::render_streak(&service.streak()?),
        Command::Version => env!("CARGO_PKG_VERSION").to_string(),
    };

    println!("{output}");
    Ok(())
}
