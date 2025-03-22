use fgcore::controllers;
use fgdb::data::{
    muscle::{MuscleData, MuscleIndexParams},
    ResponseData,
};
use tauri::{ipc::Request, State};
use validator::ValidationErrors;

use crate::state::AppState;

use super::parse_params;

#[tauri::command]
pub async fn muscle_index(
    request: Request<'_>,
    state: State<'_, AppState>,
) -> Result<ResponseData<Vec<MuscleData>>, ResponseData<ValidationErrors>> {
    // parse and pass to controller
    match parse_params::<MuscleIndexParams>(request.body().to_owned()) {
        Ok(params) => Ok(controllers::muscle::index(params, &state.dbc).await?.into()),
        Err(err) => return Ok(ResponseData::from_errors(err)),
    }
}
