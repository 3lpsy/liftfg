use anyhow::Result;
use validator::ValidationErrors;

// use fgdb::entity::{profile::data::ProfileCreateData, wrappers::RequestData};
use crate::commands::parse_params;
use crate::state::AppState;
use fgcore::controllers::profile as profile_controller;
use fgdb::data::{profile::ProfileData, profile::ProfileGetParams, ResponseData};
use tauri::{self};

// what if parsing failed on serde deserialize
#[tauri::command]
pub async fn get_profile(
    request: tauri::ipc::Request<'_>,
    state: tauri::State<'_, AppState>,
) -> Result<ResponseData<ProfileData>, ResponseData<ValidationErrors>> {
    // parse and pass to controller
    match parse_params::<ProfileGetParams>(request.body().to_owned()) {
        Ok(params) => Ok(profile_controller::get(params, &state.dbc).await?.into()),
        Err(err) => return Ok(ResponseData::new(None, Some(err))),
    }
}
#[cfg(test)]
mod tests {
    use fgdb::data::{
        profile::{ProfileData, ProfileGetParams},
        RequestableParams,
    };
    use serde_json::json;
    use tauri::ipc::InvokeBody;

    use crate::utils::testutils;
    #[tokio::test]
    async fn it_invokes_get_profile() {
        let (mut _app, webview, _test_id) = testutils::seeded_test_setup().await.unwrap();
        let payload = ProfileGetParams {
            id: None,
            name: None,
        }
        .to_params();
        let res = testutils::invoke::<ProfileData>(
            &webview,
            "get_profile",
            InvokeBody::Json(json!(payload)),
        )
        .await;
        assert!(res.data.is_some());
        assert_eq!(res.data.unwrap().name, format!("TestProfile"));
    }
}
