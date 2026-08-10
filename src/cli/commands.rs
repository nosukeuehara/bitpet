use crate::cli::CliError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Status,
    Feed,
    Play,
    Go,
    Report,
    Streak,
    Version,
}

impl Command {
    pub fn parse<I>(args: I) -> Result<Self, CliError>
    where
        I: IntoIterator<Item = String>,
    {
        let mut args = args.into_iter();
        let command = match args.next().as_deref() {
            None | Some("status") => Self::Status,
            Some("feed") => Self::Feed,
            Some("play") => Self::Play,
            Some("go") => Self::Go,
            Some("report") => Self::Report,
            Some("streak") => Self::Streak,
            Some("--version") | Some("-V") => Self::Version,
            Some(unknown) => return Err(CliError::UnknownCommand(unknown.to_string())),
        };

        Ok(command)
    }
}

#[cfg(test)]
mod tests {
    use super::Command;

    #[test]
    fn no_args_defaults_to_status() {
        let args = Vec::<String>::new();

        assert_eq!(Command::parse(args), Ok(Command::Status));
    }

    #[test]
    fn parses_status_command() {
        assert_eq!(Command::parse(["status".to_string()]), Ok(Command::Status));
    }
}
