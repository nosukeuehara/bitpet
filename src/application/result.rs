use crate::domain::action::Action;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ApplicationError {
    ActionLimitReached(Action),
    InvalidSaveData,
    SaveDirectoryUnavailable,
    Storage(String),
}

impl Display for ApplicationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ActionLimitReached(_) => {
                write!(
                    formatter,
                    "Action limit reached.\n\nMaybe try again tomorrow."
                )
            }
            Self::InvalidSaveData => {
                write!(formatter, "BitPet couldn't read your save data.")
            }
            Self::SaveDirectoryUnavailable => {
                write!(
                    formatter,
                    "BitPet couldn't determine where to store save data."
                )
            }
            Self::Storage(message) => write!(formatter, "{message}"),
        }
    }
}

impl Error for ApplicationError {}

pub type ApplicationResult<T> = Result<T, ApplicationError>;
