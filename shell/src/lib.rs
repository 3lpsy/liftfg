#![feature(stmt_expr_attributes)]

use anyhow::Result;
use config::AppConfig;
use db::db;
use fg_core::logging;
use tauri::{App, Manager};
use tracing::info;

mod config;
mod plugins;

#[tauri::command]
async fn get_user() -> Result<(), String> {
    Ok(())
}

#[tracing::instrument(skip_all, parent = None)]
fn setup(app: &mut App, log_handle: logging::LogHandle) -> Result<()> {
    let rt = tauri::async_runtime::handle();
    rt.block_on(async {
        config::setup(app).await?;

        let config = app.state::<AppConfig>();
        info!("App Config: {:?}", config);
        logging::setup_fs(&config.logs_dir, log_handle)?;
        info!("File tracing initialized");
        db::migrate(&config.db_path).await?;
        Ok::<(), anyhow::Error>(()) // Explicitly specify the Ok type here
    })?;
    info!("App setup complete");
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let log_handle = match logging::setup() {
        Ok(handle) => {
            info!("Console tracing initialized");
            handle
        }
        Err(e) => {
            panic!("Error setting up logging: {:?}", e);
        }
    };
    let mut builder = tauri::Builder::default();
    builder = plugins::setup(builder);
    builder = builder.setup(|app| Ok(setup(app, log_handle)?));
    builder = builder.invoke_handler(tauri::generate_handler![get_user]);

    builder.run(tauri::generate_context!()).unwrap_or_else(|e| {
        eprintln!("Error while running Tauri application: {:?}", e);
        panic!("Error while running Tauri application")
    });
}
