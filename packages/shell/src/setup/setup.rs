use std::fs;

use crate::{
    config::{self, AppConfig},
    state,
};

use anyhow::Result;
use chrono::Utc;
use fgcore::logging;
use fgdb::{db, seed};
use fgutils;
use fgutils::environment::Environment;
use tauri::{App, Manager, Runtime};
use tracing::{debug, error, info, warn};

#[tracing::instrument(skip_all, parent = None, target = "setup_async")]
pub async fn setup_async<R: Runtime>(
    app: &mut App<R>,
    log_handles: Option<(logging::LayersHandle, logging::FilterHandle)>,
    conf: Option<AppConfig>,
) -> Result<()> {
    let start_dt = Utc::now();
    if !AppConfig::should_no_dotenv(app)? {
        fgutils::load_dotenvs(vec![
            fgutils::cwd().join(".env"),
            app.path().app_data_dir().unwrap().join("environment"),
        ])?
    } else {
        info!("Skipping dotenv loading");
    }

    config::manage(app, conf).await?;

    let (layer_handle, filter_handle) = log_handles
        .map(|(layer, filter)| (Some(layer), Some(filter)))
        .unwrap_or((None, None));

    let config = app.state::<AppConfig>();
    if !config.no_logging_filer_reload {
        if let Some(filter_handle) = filter_handle {
            logging::reload_filter(filter_handle)?;
            debug!("Filter reloaded");
        }
    }
    // At this point the environment, logging, and config should all be setup
    // config should be the source of truth for everything
    info!("App Config: {:?}", config);
    if !config.data_dir.exists() {
        debug!("Creating data directory...",);
        fs::create_dir_all(&config.data_dir)?;
    }

    if !config.no_fs_logging {
        if let Some(layer_handle) = layer_handle {
            logging::setup_fs(&config.logs_dir, layer_handle)?;
            info!("File tracing initialized");
        }
    }

    if !config.no_migrate {
        info!("Migrating...");
        db::migrate(&config.db_path).await?;
    }

    if config.should_seed_dev {
        if config.app_env == Environment::Prod {
            error!("Cannot seed dev data in prod environment!");
        } else {
            let dbc = db::get_dbc(&config.db_path).await?;
            seed::dev(&dbc).await?;
        }
    }

    // finally intiallize state
    state::manage(app, None).await?;
    // if i ever need the webview,
    // null on mockruntime
    // let mut webview = app.webview_windows().get("main").unwrap().as_ref();

    let end_dt = Utc::now();
    let boot_duration = end_dt - start_dt;
    info!(
        "App setup complete ({} ms)",
        boot_duration.num_milliseconds()
    );
    Ok(())
}

pub fn setup<R: Runtime>(
    app: &mut App<R>,
    log_handles: Option<(logging::LayersHandle, logging::FilterHandle)>,
    conf: Option<AppConfig>,
) -> Result<()> {
    tauri::async_runtime::block_on(async { Ok(setup_async(app, log_handles, conf).await?) })
}
