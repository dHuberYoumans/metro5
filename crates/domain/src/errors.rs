use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("invalid log level")]
    InvalidLogLevel,

    #[error("invalid filter")]
    InvalidFilter,
}

pub type Result<T> = std::result::Result<T, DomainError>;
