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

    let log_handles = match logging::init() {
        Ok(handles) => {
            info!("Console tracing initialized");
            handles
        }
        Err(e) => {
            panic!("Error setting up logging: {:?}", e);
        }
    };
    let mut builder = tauri::Builder::default();
    builder = plugins::load(builder);

    // Setup Callback
    builder = builder.setup(|app| Ok(setup::setup(app, log_handles)?));
    builder = builder.invoke_handler(handlers::generate());

    builder.run(tauri::generate_context!()).unwrap_or_else(|e| {
        eprintln!("Error while running Tauri application: {:?}", e);
        panic!("Error while running Tauri application")
    });
}

#[cfg(test)]
pub mod testutils {
    use std::collections::HashMap;

    use super::{handlers, plugins};
    use anyhow::Result;
    use serde_json::json;
    use tauri::test::{mock_builder, mock_context, noop_assets, MockRuntime};
    use tauri::utils::config::PluginConfig;
    use tauri::{App, Config, Context, Runtime};

    // TOOD: create testing json file and just load that
    pub fn create_context<R: Runtime>() -> Context<R> {
        let mut plugins_config = HashMap::new();
        let cli_config = json!({
            "description": "Test CLI",
            "longDescription": "Test CLI for unit tests",
            "beforeHelp": "",
            "afterHelp": "",
            "args": [],
            "subcommands": {}
        });
        plugins_config.insert("cli".to_string(), cli_config);
        let mut context = mock_context(noop_assets());
        let config = context.config_mut();
        config.plugins = PluginConfig(plugins_config);
        context
    }

    pub fn create_app() -> Result<App<MockRuntime>> {
        std::env::set_var("APP_ENV", "test");
        std::env::set_var("DATABASE_PATH", "test.db");
        std::env::set_var("NO_DOTENV", "true");
        let mut builder = mock_builder();
        builder = plugins::load(builder);
        Ok(builder
            .setup(|_app| Ok(()))
            .invoke_handler(handlers::generate())
            .build(create_context())?)
    }
}
