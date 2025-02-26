use anyhow::Result;
use validator::ValidationErrors;

// use fgdb::entity::{profile::data::ProfileCreateData, wrappers::RequestData};
use crate::commands::parse_params;
use crate::state::AppState;
use fgcore::controllers::profile as profile_controller;
use fgdb::data::{profile::ProfileData, profile::ProfileDeleteParams, ResponseData};
use tauri::{self};

// what if parsing failed on serde deserialize
#[tauri::command]
pub async fn profile_delete(
    request: tauri::ipc::Request<'_>,
    state: tauri::State<'_, AppState>,
) -> Result<ResponseData<ProfileData>, ResponseData<ValidationErrors>> {
    // parse and pass to controller
    match parse_params::<ProfileDeleteParams>(request.body().to_owned()) {
        Ok(params) => Ok(profile_controller::delete(params, &state.dbc).await?.into()),
        Err(err) => return Ok(ResponseData::from_errors(err)),
    }
}

// TODO tests
