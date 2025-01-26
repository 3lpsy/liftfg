#![feature(stmt_expr_attributes)]

use fgcore::logging;
use tracing::info;

mod config;
pub mod handlers;
mod plugins;
mod setup;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Only env that should be read before setup is RUST_LOG
    // Revisit if there are tauri runtime variables needed

    let log_handles = match logging::setup() {
        Ok(handles) => {
            info!("Console tracing initialized");
            handles
        }
        Err(e) => {
            panic!("Error setting up logging: {:?}", e);
        }
    };
    let mut builder = tauri::Builder::default();
    builder = plugins::setup(builder);
    // Setup Callback
    builder = builder.setup(|app| Ok(setup::setup(app, log_handles)?));
    builder = builder.invoke_handler(handlers::generate());

    builder.run(tauri::generate_context!()).unwrap_or_else(|e| {
        eprintln!("Error while running Tauri application: {:?}", e);
        panic!("Error while running Tauri application")
    });
}
