use crate::commands::parse_data;
use crate::state::AppState;
use anyhow::Result;
use fgcore::controllers::profile as profile_controller;
use fgdb::data::{
    profile::{ProfileData, ProfileStoreData},
    ResponseData,
};
use tauri::{self};
use validator::ValidationErrors;

// controllers are really handlers
// handlers are still mostly controllers
#[tauri::command]
pub async fn profile_store(
    request: tauri::ipc::Request<'_>,
    state: tauri::State<'_, AppState>,
) -> Result<ResponseData<ProfileData>, ResponseData<ValidationErrors>> {
    match parse_data::<ProfileStoreData>(request.body().to_owned()) {
        Ok(data) => Ok(profile_controller::store(data, &state.dbc).await?),
        Err(err) => return Ok(ResponseData::from_errors(err)),
    }
}

#[cfg(test)]
mod tests {
    use fgdb::{
        data::{
            profile::{ProfileData, ProfileStoreData},
            RequestableData,
        },
        entity::profile,
    };
    use sea_orm::ActiveModelTrait;
    use serde_json::json;
    use std::collections::HashMap;
    use tauri::{ipc::InvokeBody, Manager};
    use validator::ValidationErrors;

    use crate::{state::AppState, utils::testutils};

    #[tokio::test(flavor = "multi_thread")]
    async fn it_invokes_profile_store() {
        let (mut _app, webview, test_id) = testutils::default_test_setup().await.unwrap();
        let payload = ProfileStoreData {
            name: test_id.to_string(),
            is_default: Some(true),
        }
        .as_request();
        let res = testutils::invoke::<ProfileData>(
            &webview,
            "profile_store",
            InvokeBody::Json(json!(payload)),
        )
        .await;
        assert!(res.data.is_some());
        assert_eq!(res.data.unwrap().name, format!("{test_id}"));
    }
    #[tokio::test(flavor = "multi_thread")] // deadlock single threaded
    async fn it_invokes_profile_store_name_collision() {
        let (app, webview, test_id) = testutils::default_test_setup().await.unwrap();
        profile::ActiveModel::from(ProfileStoreData {
            name: test_id.to_string(),
            is_default: Some(true),
        })
        .insert(&app.state::<AppState>().dbc)
        .await
        .unwrap();

        // same name, non default fails on name collision
        let payload = ProfileStoreData {
            name: test_id.to_string(),
            is_default: Some(false),
        }
        .as_request();
        // if this fails to deser, it's cause an error wasn't returned which is a faile dtest
        let res = testutils::invoke::<ValidationErrors>(
            &webview,
            "profile_store",
            InvokeBody::Json(json!(payload)),
        )
        .await;
        // assert error occured due to name conflict
        assert!(res.errors.unwrap().errors().contains_key("name"));
    }

    #[tokio::test(flavor = "multi_thread")] // deadlock single threaded
    async fn it_invokes_profile_store_double_default_toggles_other_default() {
        let (app, webview, test_id) = testutils::default_test_setup().await.unwrap();
        profile::ActiveModel::from(ProfileStoreData {
            name: test_id.to_string(),
            is_default: Some(true),
        })
        .insert(&app.state::<AppState>().dbc)
        .await
        .unwrap();

        let payload = ProfileStoreData {
            name: format!("{test_id}2"),
            is_default: Some(true),
        }
        .as_request();
        let res = testutils::invoke::<ProfileData>(
            &webview,
            "profile_store",
            InvokeBody::Json(json!(payload)),
        )
        .await;
        assert!(res.data.is_some());
        let p = profile::Entity::by_name(&app.state::<AppState>().dbc, &test_id.to_string())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(p.is_default, false);
    }

    #[tokio::test(flavor = "multi_thread")] // deadlock single threaded
    async fn it_invokes_profile_store_bad_payload() {
        let (_, webview, _) = testutils::default_test_setup().await.unwrap();

        // missing data
        let mut badpayload = HashMap::new();
        let res = testutils::invoke::<ValidationErrors>(
            &webview,
            "profile_store",
            InvokeBody::Json(json!(badpayload.clone())),
        )
        .await;
        assert!(res.errors.unwrap().errors().contains_key("data"));

        let mut dpay = HashMap::new();
        dpay.insert("x", "x");
        badpayload.insert("data", dpay);
        let res = testutils::invoke::<ValidationErrors>(
            &webview,
            "profile_store",
            InvokeBody::Json(json!(badpayload.clone())),
        )
        .await;

        assert!(ValidationErrors::has_error(
            &Err(res.errors.unwrap()),
            "name"
        ));

        let res = testutils::invoke::<ValidationErrors>(
            &webview,
            "profile_store",
            InvokeBody::Raw(vec![0xE2, 0x82]),
        )
        .await;
        assert!(ValidationErrors::has_error(
            &Err(res.errors.unwrap()),
            "request"
        ));
    }
}
