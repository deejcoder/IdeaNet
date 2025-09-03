// Domain, Application, Infrastructure, and Commands modules
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod commands;

use std::sync::Arc;
use tauri::Manager;
use application::services::database::DatabaseService;
use infrastructure::database::SurrealAdapter;
use commands::database::{db_connect, db_disconnect, db_health};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Setup database service (using in-memory database for Phase 1)
            let database_adapter = Arc::new(SurrealAdapter::new("memory".to_string()));
            let database_service = Arc::new(DatabaseService::new(database_adapter));
            
            app.manage(database_service);
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            db_connect,
            db_disconnect,
            db_health
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
