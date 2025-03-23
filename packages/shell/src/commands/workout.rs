use fgcore::controllers;
use fgdb::data::{
    workout::{WorkoutData, WorkoutIndexParams, WorkoutStoreData},
    ResponseData,
};
use tauri::{ipc::Request, State};
use validator::ValidationErrors;

use crate::state::AppState;

use super::{parse_data, parse_params};

#[tauri::command]
pub async fn workout_index(
    request: Request<'_>,
    state: State<'_, AppState>,
) -> Result<ResponseData<Vec<WorkoutData>>, ResponseData<ValidationErrors>> {
    // parse and pass to controller
    match parse_params::<WorkoutIndexParams>(request.body().to_owned()) {
        Ok(params) => Ok(controllers::workout::index(params, &state.dbc)
            .await?
            .into()),
        Err(err) => return Ok(ResponseData::from_errors(err)),
    }
}

#[tauri::command]
pub async fn workout_store(
    request: Request<'_>,
    state: State<'_, AppState>,
) -> Result<ResponseData<WorkoutData>, ResponseData<ValidationErrors>> {
    // parse and pass to controller
    match parse_data::<WorkoutStoreData>(request.body().to_owned()) {
        Ok(data) => Ok(controllers::workout::store(data, &state.dbc).await?.into()),
        Err(err) => return Ok(ResponseData::from_errors(err)),
    }
}
