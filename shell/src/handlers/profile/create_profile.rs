use crate::handlers::parse_data;
use crate::state::AppState;
use anyhow::Result;
use fgcore::controllers::profile as profile_controller;
use fgdb::{
    data::ResponseData,
    entity::profile::{self, ProfileResponseData},
};
use tauri::{self};
use validator::ValidationErrors;

// controllers are really handlers
// handlers are still mostly controllers
#[tauri::command]
pub async fn create_profile(
    request: tauri::ipc::Request<'_>,
    state: tauri::State<'_, AppState>,
) -> Result<ResponseData<ProfileResponseData>, ResponseData<ValidationErrors>> {
    // parse and pass to controller
    match parse_data::<profile::ProfileCreateData>(request.body().to_owned()) {
        Ok(data) => Ok(profile_controller::create(data, &state.dbc).await?.into()),
        Err(err) => return Ok(ResponseData::new(None, Some(err))),
    }
}

// What can go wrong?
// - Failed to preparse request - untested
// - failed to parse body
//   - missing data - tested
//   - missing data.somefield - tested
//   - utf - untested
//   - two users w/ same name - tested
//   - two users are default - tested
#[cfg(test)]
mod tests {
    use fgdb::{
        data::RequestableData,
        entity::profile::{ProfileCreateData, ProfileResponseData},
    };
    use serde_json::json;
    use std::collections::HashMap;
    use tauri::ipc::InvokeBody;
    use validator::ValidationErrors;

    use crate::utils::testutils;

    #[tokio::test]
    async fn it_invokes_create_profile() {
        let (mut _app, webview, test_id) = testutils::default_test_setup().await.unwrap();
        let payload = ProfileCreateData {
            name: test_id.to_string(),
            is_default: Some(true),
        }
        .to_request();
        let res = testutils::invoke::<ProfileResponseData>(
            &webview,
            "create_profile",
            InvokeBody::Json(json!(payload)),
        )
        .await;
        assert!(res.data.is_some());
        assert_eq!(res.data.unwrap().name, format!("{test_id}"));

        // same name, non default fails on name collision
        let payload = fgdb::entity::profile::ProfileCreateData {
            name: test_id.to_string(),
            is_default: Some(false),
        }
        .to_request();
        let res = testutils::invoke::<ValidationErrors>(
            &webview,
            "create_profile",
            InvokeBody::Json(json!(payload)),
        )
        .await;
        // assert error occured due to name conflict
        assert!(res.errors.unwrap().errors().contains_key("name"));

        // double default fails
        let payload = fgdb::entity::profile::ProfileCreateData {
            name: format!("{test_id}2"),
            is_default: Some(true),
        }
        .to_request();
        let res = testutils::invoke::<ValidationErrors>(
            &webview,
            "create_profile",
            InvokeBody::Json(json!(payload)),
        )
        .await;
        assert!(res.errors.unwrap().errors().contains_key("is_default"));

        // missing data
        let mut badpayload = HashMap::new();
        let res = testutils::invoke::<ValidationErrors>(
            &webview,
            "create_profile",
            InvokeBody::Json(json!(badpayload.clone())),
        )
        .await;
        assert!(res.errors.unwrap().errors().contains_key("data"));

        let mut dpay = HashMap::new();
        dpay.insert("x", "x");
        badpayload.insert("data", dpay);
        let res = testutils::invoke::<ValidationErrors>(
            &webview,
            "create_profile",
            InvokeBody::Json(json!(badpayload.clone())),
        )
        .await;

        assert!(ValidationErrors::has_error(
            &Err(res.errors.unwrap()),
            "name"
        ));

        let res = testutils::invoke::<ValidationErrors>(
            &webview,
            "create_profile",
            InvokeBody::Raw(vec![0xE2, 0x82]),
        )
        .await;
        assert!(ValidationErrors::has_error(
            &Err(res.errors.unwrap()),
            "request"
        ));
    }
}
