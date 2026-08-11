pub mod commands;
pub mod renderer;

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
        Command::Feed => renderer::render_not_implemented("feed"),
        Command::Play => renderer::render_not_implemented("play"),
        Command::Go => renderer::render_not_implemented("go"),
        Command::Report => renderer::render_not_implemented("report"),
        Command::Streak => renderer::render_not_implemented("streak"),
        Command::Version => env!("CARGO_PKG_VERSION").to_string(),
    };

    println!("{output}");
    Ok(())
}
