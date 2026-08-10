use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ApplicationError {
    Storage(String),
}

impl Display for ApplicationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Storage(message) => write!(formatter, "{message}"),
        }
    }
}

impl Error for ApplicationError {}

pub type ApplicationResult<T> = Result<T, ApplicationError>;
