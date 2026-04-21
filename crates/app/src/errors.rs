use domain::errors::DomainError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("unknown command")]
    UnknownCommand,

    #[error("invalid prefix")]
    InvalidCommandPrefix,

    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error("too many arguments")]
    TooManyArguments,
}

pub type Result<T> = std::result::Result<T, ApplicationError>;
