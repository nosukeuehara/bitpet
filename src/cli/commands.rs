use crate::cli::CliError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Status,
    Feed,
    Play,
    Go,
    Report,
    Streak,
    Help(Option<HelpTopic>),
    Version,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HelpTopic {
    Status,
    Feed,
    Play,
    Go,
    Report,
    Streak,
}

impl Command {
    pub fn parse<I>(args: I) -> Result<Self, CliError>
    where
        I: IntoIterator<Item = String>,
    {
        let args = args.into_iter().collect::<Vec<_>>();
        let command = match args.as_slice() {
            [] => Self::Status,
            [flag] if is_help_flag(flag) => Self::Help(None),
            [flag] if is_version_flag(flag) => Self::Version,
            [command] => command_from_str(command)?,
            [command, flag] if command == "help" && is_help_flag(flag) => Self::Help(None),
            [command, flag] if is_help_flag(flag) => {
                Self::Help(Some(help_topic_from_str(command)?))
            }
            [command, topic] if command == "help" => Self::Help(Some(help_topic_from_str(topic)?)),
            [unknown, ..] => return Err(CliError::UnknownCommand(unknown.to_string())),
        };

        Ok(command)
    }
}

fn command_from_str(command: &str) -> Result<Command, CliError> {
    match command {
        "status" => Ok(Command::Status),
        "feed" => Ok(Command::Feed),
        "play" => Ok(Command::Play),
        "go" => Ok(Command::Go),
        "report" => Ok(Command::Report),
        "streak" => Ok(Command::Streak),
        "help" => Ok(Command::Help(None)),
        "--version" | "-V" => Ok(Command::Version),
        unknown => Err(CliError::UnknownCommand(unknown.to_string())),
    }
}

fn help_topic_from_str(command: &str) -> Result<HelpTopic, CliError> {
    match command {
        "status" => Ok(HelpTopic::Status),
        "feed" => Ok(HelpTopic::Feed),
        "play" => Ok(HelpTopic::Play),
        "go" => Ok(HelpTopic::Go),
        "report" => Ok(HelpTopic::Report),
        "streak" => Ok(HelpTopic::Streak),
        unknown => Err(CliError::UnknownCommand(unknown.to_string())),
    }
}

fn is_help_flag(value: &str) -> bool {
    matches!(value, "--help" | "-h")
}

fn is_version_flag(value: &str) -> bool {
    matches!(value, "--version" | "-V")
}

#[cfg(test)]
mod tests {
    use super::{Command, HelpTopic};

    #[test]
    fn no_args_defaults_to_status() {
        let args = Vec::<String>::new();

        assert_eq!(Command::parse(args), Ok(Command::Status));
    }

    #[test]
    fn parses_status_command() {
        assert_eq!(Command::parse(["status".to_string()]), Ok(Command::Status));
    }

    #[test]
    fn parses_help_flags() {
        assert_eq!(
            Command::parse(["--help".to_string()]),
            Ok(Command::Help(None))
        );
        assert_eq!(Command::parse(["-h".to_string()]), Ok(Command::Help(None)));
    }

    #[test]
    fn parses_subcommand_help() {
        assert_eq!(
            Command::parse(["status".to_string(), "--help".to_string()]),
            Ok(Command::Help(Some(HelpTopic::Status)))
        );
        assert_eq!(
            Command::parse(["feed".to_string(), "-h".to_string()]),
            Ok(Command::Help(Some(HelpTopic::Feed)))
        );
        assert_eq!(
            Command::parse(["help".to_string(), "go".to_string()]),
            Ok(Command::Help(Some(HelpTopic::Go)))
        );
    }

    #[test]
    fn parses_version_flags() {
        assert_eq!(
            Command::parse(["--version".to_string()]),
            Ok(Command::Version)
        );
        assert_eq!(Command::parse(["-V".to_string()]), Ok(Command::Version));
    }
}
