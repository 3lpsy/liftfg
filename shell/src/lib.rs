// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use anyhow::{Error, Result};
use db::db;
use fg_core::logging;
use serde_json::Value;
use std::env;
// use std::option::Option;
use tauri::{App, Manager};
use tauri_plugin_cli::CliExt;
// use tauri_plugin_fs::FsExt;
use tracing::{debug, info};

#[tauri::command]
async fn get_user() -> Result<(), String> {
    Ok(())
}

pub async fn setup(app: &mut App) -> Result<(), Error> {
    let app_data_dir = app.path().app_data_dir().unwrap();
    let log_dir = app_data_dir.join("logs");
    logging::setup(&log_dir)?;
    debug!("Logging initialized: {}", log_dir.display());
    match app.cli().matches() {
        Ok(matches) => {
            let args = matches.args;

            let args_db = args.get("db");
            let db_path: String;
            if args_db.is_some() && args_db.unwrap().value != Value::Null {
                db_path = args_db.unwrap().value.as_str().unwrap().to_owned();
            } else if env::var("DATABASE_PATH").is_ok()
                && env::var("DATABASE_PATH").unwrap().len() > 0
            {
                db_path = env::var("DATABASE_PATH").unwrap();
            } else {
                println!("C");
                let db_path_buf = app_data_dir.join("app.db");
                db_path = db_path_buf.to_string_lossy().to_string();
            }

            db::migrate(Some(&db_path)).await?;
        }
        Err(_e) => {}
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let rt = tauri::async_runtime::handle();
            Ok(rt.block_on(async { setup(app).await.unwrap() }))
        })
        .invoke_handler(tauri::generate_handler![get_user])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
