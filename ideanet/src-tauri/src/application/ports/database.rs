use async_trait::async_trait;
use crate::domain::errors::DatabaseError;

#[derive(Debug, Clone)]
pub struct DatabaseHealth {
    pub connected: bool,
    pub message: String,
}

#[async_trait]
pub trait DatabasePort: Send + Sync {
    async fn connect(&self) -> Result<(), DatabaseError>;
    async fn disconnect(&self) -> Result<(), DatabaseError>;
    async fn health_check(&self) -> Result<DatabaseHealth, DatabaseError>;
}
