use fgcore::controllers;
use fgdb::data::{
    profile_workout::{ProfileWorkoutData, ProfileWorkoutDeleteData, ProfileWorkoutStoreData},
    ResponseData,
};
use tauri::{ipc::Request, State};
use validator::ValidationErrors;

use crate::state::AppState;

use super::parse_data;

#[tauri::command]
pub async fn profile_workout_store(
    request: Request<'_>,
    state: State<'_, AppState>,
) -> Result<ResponseData<ProfileWorkoutData>, ResponseData<ValidationErrors>> {
    match parse_data::<ProfileWorkoutStoreData>(request.body().to_owned()) {
        Ok(data) => Ok(controllers::profile_workout::store(data, &state.dbc)
            .await?
            .into()),
        Err(err) => return Ok(ResponseData::from_errors(err)),
    }
}

#[tauri::command]
pub async fn profile_workout_delete(
    request: Request<'_>,
    state: State<'_, AppState>,
) -> Result<ResponseData<ProfileWorkoutData>, ResponseData<ValidationErrors>> {
    match parse_data::<ProfileWorkoutDeleteData>(request.body().to_owned()) {
        Ok(data) => Ok(controllers::profile_workout::delete(data, &state.dbc)
            .await?
            .into()),
        Err(err) => return Ok(ResponseData::from_errors(err)),
    }
}
