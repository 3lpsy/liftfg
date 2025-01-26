use crate::config::{self, AppConfig};

use anyhow::Result;
use db::db;
use fgcore::{logging, utils};
use tauri::{App, Manager, Runtime};
use tracing::info;

#[tracing::instrument(skip_all, parent = None)]
pub fn setup<R: Runtime>(
    app: &mut App<R>,
    log_handles: (logging::LayersHandle, logging::FilterHandle),
) -> Result<()> {
    tracing::debug!("Setup called");

    if !AppConfig::should_skip_dotenv(app) {
        // load the env from dev if it exists
        // load the environemnt from app_data_dir if exists
        utils::load_dotenvs(vec![
            utils::cwd().join(".env"),
            app.path().app_data_dir().unwrap().join("environment"),
        ])?
    }
    logging::reload_filter(log_handles.1)?;
    let rt = tauri::async_runtime::handle();
    rt.block_on(async {
        tracing::debug!("Before config setup");
        config::setup(app).await?;
        let config = app.state::<AppConfig>();
        info!("App Config: {:?}", config);
        logging::setup_fs(&config.logs_dir, log_handles.0)?;
        info!("File tracing initialized");
        db::migrate(&config.db_path).await?;
        // seed data?q
        Ok::<(), anyhow::Error>(()) // Explicitly specify the Ok type here
    })?;
    info!("App setup complete");
    Ok(())
}
