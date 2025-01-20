#![feature(stmt_expr_attributes)]

use anyhow::Result;
use config::AppConfig;
use db::db;
use fg_core::logging;
use std::env;
use tauri::Manager;
use tracing::info;

mod config;
mod plugins;

#[tauri::command]
async fn get_user() -> Result<(), String> {
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();
    builder = plugins::setup(builder);

    builder = builder.setup(|app| {
        let rt = tauri::async_runtime::handle();
        let r = rt.block_on(async {
            if let Err(e) = config::setup(app).await {
                eprintln!("Failed to set up config: {:?}", e);
            }

            let config = app.state::<AppConfig>();
            dbg!(&config);
            if let Err(e) = logging::setup(&config.logs_dir) {
                eprintln!("Failed to set up logging: {:?}", e);
            }

            if let Err(e) = db::migrate(&config.db_path).await {
                eprintln!("Failed to migrate database: {:?}", e);
            }
            Ok(())
        });
        info!("App setup complete");
        return r;
    });

    builder = builder.invoke_handler(tauri::generate_handler![get_user]);

    builder.run(tauri::generate_context!()).unwrap_or_else(|e| {
        eprintln!("Error while running Tauri application: {:?}", e);
        panic!("Error while running Tauri application")
    });
}
