use anyhow::Error;
use fgdb::db::get_dbc;
use sea_orm::DatabaseConnection;
use tauri::{App, Manager, Runtime};

use crate::config::AppConfig;

pub async fn manage<R: Runtime>(app: &mut App<R>, state: Option<AppState>) -> Result<(), Error> {
    let state_: AppState;
    if state.is_none() {
        state_ = AppState::from_app(app).await?;
    } else {
        state_ = state.unwrap();
    }
    app.manage(state_);
    Ok(())
}

// Configuration structure
pub struct AppState {
    pub dbc: DatabaseConnection,
}

impl AppState {
    pub async fn from_app<R: Runtime>(app: &App<R>) -> Result<Self, Error> {
        let config = app.state::<AppConfig>();
        // db_path has been resolved
        let dbc = get_dbc(&config.db_path).await?;
        Ok(AppState { dbc })
    }
}
