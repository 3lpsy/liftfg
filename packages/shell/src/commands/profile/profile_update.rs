use crate::commands::parse_data;
use crate::state::AppState;
use anyhow::Result;
use fgcore::controllers::profile as profile_controller;
use fgdb::data::{
    profile::{ProfileData, ProfileUpdateData},
    ResponseData,
};
use tauri::{self};
use validator::ValidationErrors;

// controllers are really handlers
// handlers are still mostly controllers
#[tauri::command]
pub async fn profile_update(
    request: tauri::ipc::Request<'_>,
    state: tauri::State<'_, AppState>,
) -> Result<ResponseData<ProfileData>, ResponseData<ValidationErrors>> {
    // parse and pass to controller
    match parse_data::<ProfileUpdateData>(request.body().to_owned()) {
        Ok(data) => Ok(profile_controller::update(data, &state.dbc).await?),
        Err(err) => return Ok(ResponseData::from_errors(err)),
    }
}
