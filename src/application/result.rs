use crate::domain::action::Action;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ApplicationError {
    ActionLimitReached(Action),
    InvalidSaveData,
    InvalidAction,
    ExpeditionLocked,
    PetAway,
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
            Self::InvalidAction => {
                write!(formatter, "That action is not available.")
            }
            Self::ExpeditionLocked => {
                write!(formatter, "Mochi is not ready to explore yet.")
            }
            Self::PetAway => {
                write!(
                    formatter,
                    "Mochi is exploring.\n\nPlease wait until Mochi returns."
                )
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
