use std::sync::Arc;

use crate::application::ports::database::{DatabasePort, DatabaseHealth};
use crate::domain::errors::DatabaseError;

pub struct DatabaseService {
    database_adapter: Arc<dyn DatabasePort>,
}

impl DatabaseService {
    pub fn new(database_adapter: Arc<dyn DatabasePort>) -> Self {
        Self { database_adapter }
    }

    pub async fn connect(&self) -> Result<(), DatabaseError> {
        self.database_adapter.connect().await
    }

    pub async fn disconnect(&self) -> Result<(), DatabaseError> {
        self.database_adapter.disconnect().await
    }

    pub async fn get_health(&self) -> Result<DatabaseHealth, DatabaseError> {
        self.database_adapter.health_check().await
    }
}
