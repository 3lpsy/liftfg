use anyhow::Result;

// use fgdb::entity::{profile::data::ProfileCreateData, wrappers::RequestData};
use crate::state::AppState;
use fgcore::controllers::profile as profile_controller;
use fgdb::entity::{
    profile::{self, ProfileResponseData},
    wrappers::ResponseData,
};
use tauri::{self};

// what if parsing failed on serde deserialize
#[tauri::command]
pub async fn create_profile(
    request: tauri::ipc::Request<'_>,
    state: tauri::State<'_, AppState>,
) -> Result<ResponseData<ProfileResponseData>, String> {
    // load body data
    let data = super::parse_body::<profile::ProfileCreateData>(request.body().to_owned());
    if data.is_err() {
        return Ok(ResponseData::new(None, Some(data.unwrap_err())));
    }
    let response = profile_controller::create_profile(data.unwrap(), &state.dbc)
        .await
        .map_err(|e| e.to_string())?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use fgdb::entity::{profile::ProfileResponseData, wrappers::ResponseData};
    // use fgdb::entity::{profile::ProfileResponseData, wrappers::ResponseData};
    use serde_json::json;
    use tracing::warn;
    use validator::ValidationErrors;

    use crate::testutils;

    #[tokio::test]
    async fn it_invokes_create_profile() {
        let (mut _app, webview, test_id) = testutils::default_test_setup().await.unwrap();

        let payload = fgdb::entity::profile::ProfileCreateData {
            name: test_id.to_string(),
            is_default: Some(true),
        };

        let res = tauri::test::get_ipc_response(
            &webview,
            tauri::webview::InvokeRequest {
                cmd: "create_profile".into(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                url: "tauri://localhost".parse().unwrap(),
                body: tauri::ipc::InvokeBody::Json(json!(payload)),
                headers: Default::default(),
                invoke_key: tauri::test::INVOKE_KEY.to_string(),
            },
        )
        .map(|b| {
            b.deserialize::<ResponseData<ProfileResponseData>>()
                .unwrap()
        })
        .unwrap();

        assert!(res.data.is_some());
        assert_eq!(res.data.unwrap().name, format!("{test_id}"));

        // same name, non default fails on name collision
        let payload = fgdb::entity::profile::ProfileCreateData {
            name: test_id.to_string(),
            is_default: Some(false),
        };
        let res = tauri::test::get_ipc_response(
            &webview,
            tauri::webview::InvokeRequest {
                cmd: "create_profile".into(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                url: "tauri://localhost".parse().unwrap(),
                body: tauri::ipc::InvokeBody::Json(json!(payload)),
                headers: Default::default(),
                invoke_key: tauri::test::INVOKE_KEY.to_string(),
            },
        )
        .map(|b| {
            b.deserialize::<ResponseData<ProfileResponseData>>()
                .unwrap()
        })
        .unwrap();
        // assert error occured due to name conflict
        assert!(res.errors.unwrap().errors().contains_key("request"));

        // double default fails
        let payload = fgdb::entity::profile::ProfileCreateData {
            name: format!("{test_id}2"),
            is_default: Some(true),
        };
        let res = tauri::test::get_ipc_response(
            &webview,
            tauri::webview::InvokeRequest {
                cmd: "create_profile".into(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                url: "tauri://localhost".parse().unwrap(),
                body: tauri::ipc::InvokeBody::Json(json!(payload)),
                headers: Default::default(),
                invoke_key: tauri::test::INVOKE_KEY.to_string(),
            },
        )
        .map(|b| {
            b.deserialize::<ResponseData<ProfileResponseData>>()
                .unwrap()
        })
        .unwrap();
        warn!("{:?}", &res);
        // assert error occured
        assert!(res.errors.unwrap().errors().contains_key("request"));

        // pre-validation parsing fails
        let mut badpayload = HashMap::new();
        badpayload.insert("x", "x");
        let res = tauri::test::get_ipc_response(
            &webview,
            tauri::webview::InvokeRequest {
                cmd: "create_profile".into(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                url: "tauri://localhost".parse().unwrap(),
                body: tauri::ipc::InvokeBody::Json(json!(badpayload)),
                headers: Default::default(),
                invoke_key: tauri::test::INVOKE_KEY.to_string(),
            },
        )
        .map(|b| {
            b.deserialize::<ResponseData<ProfileResponseData>>()
                .unwrap()
        })
        .unwrap();
        assert!(res.data.is_none());
        let verrors = res.errors.unwrap();
        assert!(ValidationErrors::has_error(&Err(verrors), "name"));
    }
}
