mod ai;
mod commands;
mod config;
mod db;
mod server;

use db::Database;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Привет, {}! Приветствие от Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_data_dir: PathBuf = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app data dir");

            let db = Database::new(app_data_dir)
                .expect("failed to initialize database");

            let db = Arc::new(db);
            app.manage(db.clone());

            server::start(db);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::get_status,
            commands::get_tabs,
            commands::analyze::get_providers,
            commands::analyze::get_config,
            commands::analyze::set_provider,
            commands::analyze::set_active_provider,
            commands::analyze::analyze_tabs,
            commands::purgatory::get_purgatory_batch,
            commands::purgatory::submit_review,
            commands::purgatory::get_review_history,
            commands::purgatory::get_purgatory_config,
            commands::purgatory::set_purgatory_config,
            commands::export::get_export_payload,
            commands::export::export_html,
            commands::export::get_export_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
