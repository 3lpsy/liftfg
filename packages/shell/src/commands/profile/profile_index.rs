use anyhow::Result;
use validator::ValidationErrors;

// use fgdb::entity::{profile::data::ProfileCreateData, wrappers::RequestData};
use crate::commands::parse_params;
use crate::state::AppState;
use fgcore::controllers::profile as profile_controller;
use fgdb::data::{profile::ProfileData, DefaultParams, ResponseData};
use tauri::{self};

// what if parsing failed on serde deserialize
#[tauri::command]
pub async fn profile_index(
    request: tauri::ipc::Request<'_>,
    state: tauri::State<'_, AppState>,
) -> Result<ResponseData<Vec<ProfileData>>, ResponseData<ValidationErrors>> {
    // parse and pass to controller
    match parse_params::<DefaultParams>(request.body().to_owned()) {
        Ok(params) => Ok(profile_controller::index(params, &state.dbc).await?.into()),
        Err(err) => return Ok(ResponseData::from_errors(err)),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::AppConfig, utils::testutils};
    use fgdb::{
        data::{Pagination, RequestableParams},
        db,
        entity::profile,
    };
    use sea_orm::{EntityTrait, Set};
    use serde_json::json;
    use tauri::{ipc::InvokeBody, Manager};

    #[tokio::test]
    async fn it_invokes_profile_index() {
        let (mut _app, webview, _test_id) = testutils::seeded_dev_test_setup().await.unwrap();
        let payload = DefaultParams {
            pagination: None,
            ..Default::default()
        }
        .as_params();
        let res = testutils::invoke::<Vec<ProfileData>>(
            &webview,
            "profile_index",
            InvokeBody::Json(json!(payload)),
        )
        .await;
        assert!(res.data.is_some());
        // seeded_test_setup creates two profiles
        assert_eq!(res.data.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn it_invokes_profile_index_pagination() {
        // no seed of two profiles
        let (app, webview, _test_id) = testutils::default_test_setup().await.unwrap();
        let mut payload = DefaultParams {
            pagination: Some(Pagination {
                page: 0,
                size: 10,
                ..Default::default()
            }),
            ..Default::default()
        }
        .as_params();
        let res = testutils::invoke::<Vec<ProfileData>>(
            &webview,
            "profile_index",
            InvokeBody::Json(json!(&payload)),
        )
        .await;
        assert!(res.data.is_some());
        assert_eq!(res.data.unwrap().len(), 0);
        let profiles: Vec<profile::ActiveModel> = (1..=15)
            .map(|i| profile::ActiveModel {
                name: Set(format!("TestProfile{}", i)),
                is_default: Set(false),
                ..Default::default()
            })
            .collect();
        let config = app.state::<AppConfig>();
        let dbc = db::get_dbc(&config.db_path).await.unwrap();
        profile::Entity::insert_many(profiles)
            .exec(&dbc)
            .await
            .unwrap();
        let res = testutils::invoke::<Vec<ProfileData>>(
            &webview,
            "profile_index",
            InvokeBody::Json(json!(&payload)),
        )
        .await;
        assert!(res.data.is_some());
        assert_eq!(res.data.unwrap().len(), 10);
        // second page, 0 index
        payload
            .params
            .as_mut()
            .unwrap()
            .pagination
            .as_mut()
            .unwrap()
            .page = 1;

        let res = testutils::invoke::<Vec<ProfileData>>(
            &webview,
            "profile_index",
            InvokeBody::Json(json!(&payload)),
        )
        .await;
        assert!(res.data.is_some());
        assert_eq!(res.data.unwrap().len(), 5);
    }
}
