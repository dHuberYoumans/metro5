use app::errors::ApplicationError;
use domain::errors::DomainError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TuiError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error(transparent)]
    Application(#[from] ApplicationError),
}

pub type Result<T> = std::result::Result<T, TuiError>;
