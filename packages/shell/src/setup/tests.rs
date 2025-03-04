#[cfg(test)]
mod tests {
    use std::time::Duration;

    use serde_json::json;
    use tokio::time;
    use tracing::info;
    use tracing::warn;

    use crate::commands;
    use crate::config::AppConfig;
    use crate::plugins;
    use crate::setup;

    use fgdb::db::get_dbc;
    use fgdb::migration;

    use fgcore::logging;
    use sea_orm::ConnectionTrait;
    use sea_orm::QueryResult;
    use sea_orm_migration::MigratorTrait;
    use tauri::Listener;
    use uuid::Uuid;
    // use std::path::PathBuf;
    use crate::utils::testutils;
    use tauri::test::mock_context;
    use tauri::test::noop_assets;
    use tauri::test::{mock_builder, MockRuntime};
    use tauri::Manager;

    #[tokio::test]
    async fn it_setups() {
        let test_id = Uuid::new_v4();
        let mut app = testutils::create_app().expect("Could not create test app");
        let config = testutils::create_config(test_id);
        setup::setup_async(
            &mut app,
            testutils::LOGGING_HANDLES.get().unwrap().to_owned(),
            Some(config),
        )
        .await
        .unwrap();
        let config = app.state::<AppConfig>();
        assert!(config.db_path.exists());
        assert!(!config.logs_dir.exists());
        let connection = get_dbc(&config.db_path).await.unwrap();
        let query = sea_orm::Statement::from_string(
            connection.get_database_backend(),
            "SELECT COUNT(*) as count FROM seaql_migrations",
        );
        let result: QueryResult = connection.query_one(query).await.unwrap().unwrap();
        let count: i32 = result.try_get("", "count").unwrap();
        assert_eq!(count, migration::Migrator::migrations().len() as i32);
    }
    #[tokio::test]
    async fn it_seeds_dev() {
        let test_id = Uuid::new_v4();
        // todo: if no fs logging and no env reload, make handles optional in setup
        let mut app = testutils::create_app().expect("Could not create test app");
        let mut config = testutils::create_config(test_id);
        config.should_seed_dev = true;
        setup::setup_async(
            &mut app,
            testutils::LOGGING_HANDLES.get().unwrap().to_owned(),
            Some(config),
        )
        .await
        .unwrap();
        let config = app.state::<AppConfig>();
        let connection = get_dbc(&config.db_path).await.unwrap();
        let query = sea_orm::Statement::from_string(
            connection.get_database_backend(),
            "SELECT COUNT(*) as count FROM profile",
        );
        let result: QueryResult = connection.query_one(query).await.unwrap().unwrap();
        let count: i32 = result.try_get("", "count").unwrap();
        assert_eq!(count, 2);
    }
    #[tokio::test]
    async fn it_invokes_log() {
        let test_id = Uuid::new_v4();
        let (mut app, webview) = testutils::create_app_and_webview().unwrap();
        setup::setup_async(
            &mut app,
            testutils::LOGGING_HANDLES.get().unwrap().to_owned(),
            Some(testutils::create_config(test_id)),
        )
        .await
        .unwrap();
        let payload = tauri::ipc::InvokeBody::Json(json!({
            "level": tauri_plugin_log::LogLevel::Debug, // wants u16
            "message": "Test Log Message"
        }));
        let res = tauri::test::get_ipc_response(
            &webview,
            tauri::webview::InvokeRequest {
                cmd: "plugin:log|log".into(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                url: "tauri://localhost".parse().unwrap(),
                body: payload,
                headers: Default::default(),
                invoke_key: tauri::test::INVOKE_KEY.to_string(),
            },
        );
        assert!(res.is_ok());
    }
}
