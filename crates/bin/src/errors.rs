use app::errors::ApplicationError;
use domain::errors::DomainError;
use infra::errors::InfraError;
use thiserror::Error;
use tui::errors::TuiError;

#[derive(Debug, Error)]
pub enum MainError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error(transparent)]
    Application(#[from] ApplicationError),

    #[error(transparent)]
    Infra(#[from] InfraError),

    #[error(transparent)]
    Tui(#[from] TuiError),
}

pub type Result<T> = std::result::Result<T, MainError>;
