use crate::config::{self, AppConfig};

use anyhow::Result;
use db::db;
use fgcore::{logging, utils};
use tauri::{App, Manager, Runtime};
use tracing::info;

#[tracing::instrument(skip_all, parent = None)]
pub async fn setup_async<R: Runtime>(
    app: &mut App<R>,
    log_handles: (logging::LayersHandle, logging::FilterHandle),
) -> Result<()> {
    if !AppConfig::should_skip_dotenv(app) {
        utils::load_dotenvs(vec![
            utils::cwd().join(".env"),
            app.path().app_data_dir().unwrap().join("environment"),
        ])?
    } else {
        info!("Skipping dotenv loading");
    }

    logging::reload_filter(log_handles.1)?;

    config::setup(app).await?;

    let config = app.state::<AppConfig>();
    info!("App Config: {:?}", config);

    logging::setup_fs(&config.logs_dir, log_handles.0)?;
    info!("File tracing initialized");

    db::migrate(&config.db_path).await?;
    info!("App setup complete");

    Ok(())
}

pub fn setup<R: Runtime>(
    app: &mut App<R>,
    log_handles: (logging::LayersHandle, logging::FilterHandle),
) -> Result<()> {
    tauri::async_runtime::block_on(async { Ok(setup_async(app, log_handles).await?) })
}
