use fgcore::controllers;
use fgdb::data::{program::ProgramData, DefaultPaginationParams, ResponseData};
use tauri::{ipc::Request, State};
use validator::ValidationErrors;

use crate::state::AppState;

use super::parse_params;

#[tauri::command]
pub async fn program_index(
    request: Request<'_>,
    state: State<'_, AppState>,
) -> Result<ResponseData<Vec<ProgramData>>, ResponseData<ValidationErrors>> {
    // parse and pass to controller
    match parse_params::<DefaultPaginationParams>(request.body().to_owned()) {
        Ok(params) => Ok(controllers::program::index(params, &state.dbc)
            .await?
            .into()),
        Err(err) => return Ok(ResponseData::from_errors(err)),
    }
}
