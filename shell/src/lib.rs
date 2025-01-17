// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use anyhow::{Error, Result};
use core;
use db::db;
use serde_json::Value;
use std::env;
use tauri_plugin_cli::CliExt;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!!", name)
}

#[tauri::command]
fn get_user() -> Result<()> {
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
async fn main() -> tauri::Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_cli::init())
        .setup(|app| {
            // Get the app config directory for logs
            let mut app_dir = app.path_resolver().app_dir()?;
            let log_dir = app_dir.join("logs");
            core::logging::setup(log_dir);

            // Create logs directory
            let log_dir = app_dir.join("logs");
            match app.cli().matches() {
                Ok(matches) => {
                    let args = matches.args;
                    let db = match args.get("db") {
                        Some(db_) => Ok(db_.value),
                        None => match env::var("DATABASE_PATH") {
                            Ok(db_env) => Ok(Value::String(db_env)),
                            Err(_e) => Err(Error::msg(
                                "No database path provided in config or environment.",
                            )),
                        },
                    }?;
                    db::migrate(Some(db.into())).await?;
                }
                Err(_) => {}
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_user])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
