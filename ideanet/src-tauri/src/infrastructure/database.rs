use async_trait::async_trait;
use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::application::ports::database::{DatabasePort, DatabaseHealth};
use crate::domain::errors::DatabaseError;

pub struct SurrealAdapter {
    db: Arc<Mutex<Option<Surreal<Db>>>>,
    db_path: String,
}

impl SurrealAdapter {
    pub fn new(db_path: String) -> Self {
        Self {
            db: Arc::new(Mutex::new(None)),
            db_path,
        }
    }

    pub async fn get_db_path() -> Result<String, DatabaseError> {
        let app_data = std::env::var("APPDATA")
            .map_err(|_| DatabaseError::ConfigurationError("Could not find APPDATA directory".to_string()))?;
        
        let db_dir = std::path::Path::new(&app_data).join("ideanet");
        
        // Create directory if it doesn't exist
        if !db_dir.exists() {
            std::fs::create_dir_all(&db_dir)
                .map_err(|e| DatabaseError::ConfigurationError(format!("Failed to create database directory: {}", e)))?;
        }
        
        Ok(db_dir.join("ideanet.db").to_string_lossy().to_string())
    }
}

#[async_trait]
impl DatabasePort for SurrealAdapter {
    async fn connect(&self) -> Result<(), DatabaseError> {
        let mut db_guard = self.db.lock().await;
        
        // If already connected, return success
        if db_guard.is_some() {
            return Ok(());
        }

        // Connect to SurrealDB (in-memory for simplicity in Phase 1)
        let db = Surreal::new::<Mem>(())
            .await
            .map_err(|e| DatabaseError::ConnectionFailed(format!("Failed to create database connection: {}", e)))?;

        // Use namespace and database
        db.use_ns("ideanet").use_db("main")
            .await
            .map_err(|e| DatabaseError::ConnectionFailed(format!("Failed to set namespace/database: {}", e)))?;

        *db_guard = Some(db);
        Ok(())
    }

    async fn disconnect(&self) -> Result<(), DatabaseError> {
        let mut db_guard = self.db.lock().await;
        *db_guard = None;
        Ok(())
    }

    async fn health_check(&self) -> Result<DatabaseHealth, DatabaseError> {
        let db_guard = self.db.lock().await;
        
        match db_guard.as_ref() {
            Some(_) => Ok(DatabaseHealth {
                connected: true,
                message: "Database connection is healthy".to_string(),
            }),
            None => Ok(DatabaseHealth {
                connected: false,
                message: "Database is not connected".to_string(),
            }),
        }
    }
}
