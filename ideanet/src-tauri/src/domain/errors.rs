use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    
    #[error("Database operation failed: {0}")]
    OperationFailed(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}
