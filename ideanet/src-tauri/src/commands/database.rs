use serde::{Deserialize, Serialize};
use tauri::State;
use std::sync::Arc;

use crate::application::services::database::DatabaseService;
use crate::application::ports::database::DatabaseHealth;

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseHealthResponse {
    pub connected: bool,
    pub message: String,
}

impl From<DatabaseHealth> for DatabaseHealthResponse {
    fn from(health: DatabaseHealth) -> Self {
        Self {
            connected: health.connected,
            message: health.message,
        }
    }
}

#[tauri::command]
pub async fn db_connect(
    database_service: State<'_, Arc<DatabaseService>>,
) -> Result<(), String> {
    database_service
        .connect()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_disconnect(
    database_service: State<'_, Arc<DatabaseService>>,
) -> Result<(), String> {
    database_service
        .disconnect()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_health(
    database_service: State<'_, Arc<DatabaseService>>,
) -> Result<DatabaseHealthResponse, String> {
    database_service
        .get_health()
        .await
        .map(DatabaseHealthResponse::from)
        .map_err(|e| e.to_string())
}
