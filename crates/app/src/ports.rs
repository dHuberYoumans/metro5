use crate::errors::ApplicationError;
use async_trait::async_trait;

#[async_trait]
pub trait MetroController: Send + Sync {
    async fn send_command(&mut self, cmd: &str) -> Result<(), ApplicationError>;
    async fn kill(&mut self) -> Result<(), ApplicationError>;
}
