use anyhow::Result;

// use fgdb::entity::{user::data::UserCreateData, wrappers::RequestData};
use crate::state::AppState;
use fgcore::controllers::user as user_controller;
use fgdb::entity::{
    user::{self, UserResponseData},
    wrappers::ResponseData,
};
use tauri::{self};

// what if parsing failed on serde deserialize
#[tauri::command]
pub async fn create_user(
    request: tauri::ipc::Request<'_>,
    state: tauri::State<'_, AppState>,
) -> Result<ResponseData<UserResponseData>, String> {
    // load body data
    let data = super::parse_body::<user::UserCreateData>(request.body().to_owned());
    if data.is_err() {
        return Ok(ResponseData::new(None, Some(data.unwrap_err())));
    }
    let response = user_controller::create_user(data.unwrap(), &state.dbc)
        .await
        .map_err(|e| e.to_string())?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use fgdb::entity::{user::UserResponseData, wrappers::ResponseData};
    // use fgdb::entity::{user::UserResponseData, wrappers::ResponseData};
    use serde_json::json;
    use validator::ValidationErrors;

    use crate::testutils;

    #[tokio::test]
    async fn it_invokes_create_user() {
        let (mut _app, webview, test_id) = testutils::default_test_setup().await.unwrap();

        let payload = fgdb::entity::user::UserCreateData {
            name: test_id.to_string(),
            email: format!("{test_id}@localhost.localhost"),
        };

        let res = tauri::test::get_ipc_response(
            &webview,
            tauri::webview::InvokeRequest {
                cmd: "create_user".into(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                url: "tauri://localhost".parse().unwrap(),
                body: tauri::ipc::InvokeBody::Json(json!(payload)),
                headers: Default::default(),
                invoke_key: tauri::test::INVOKE_KEY.to_string(),
            },
        )
        .map(|b| b.deserialize::<ResponseData<UserResponseData>>().unwrap())
        .unwrap();

        assert!(res.data.is_some());
        assert_eq!(
            res.data.unwrap().email,
            format!("{test_id}@localhost.localhost")
        );

        // create the same user and fail
        let res = tauri::test::get_ipc_response(
            &webview,
            tauri::webview::InvokeRequest {
                cmd: "create_user".into(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                url: "tauri://localhost".parse().unwrap(),
                body: tauri::ipc::InvokeBody::Json(json!(payload)),
                headers: Default::default(),
                invoke_key: tauri::test::INVOKE_KEY.to_string(),
            },
        )
        .map(|b| b.deserialize::<ResponseData<UserResponseData>>().unwrap())
        .unwrap();
        // assert error occured
        assert!(res.data.is_none());
        assert!(res.errors.is_some());
        assert!(res.errors.unwrap().errors().contains_key("request"));

        let mut badpayload = HashMap::new();
        badpayload.insert("x", "x");
        let res = tauri::test::get_ipc_response(
            &webview,
            tauri::webview::InvokeRequest {
                cmd: "create_user".into(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                url: "tauri://localhost".parse().unwrap(),
                body: tauri::ipc::InvokeBody::Json(json!(badpayload)),
                headers: Default::default(),
                invoke_key: tauri::test::INVOKE_KEY.to_string(),
            },
        )
        .map(|b| b.deserialize::<ResponseData<UserResponseData>>().unwrap())
        .unwrap();
        assert!(res.data.is_none());
        let verrors = res.errors.unwrap();
        assert!(ValidationErrors::has_error(&Err(verrors), "email"));
    }
}
