use domain::errors::DomainError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InfraError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Domain(#[from] DomainError),
}

pub type Result<T> = std::result::Result<T, InfraError>;
