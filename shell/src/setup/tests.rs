#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::time;

    use crate::config::AppConfig;
    use crate::handlers;
    use crate::plugins;
    use crate::setup;
    use crate::testutils::create_config;

    use fgdb::db::get_dbc;
    use fgdb::migration;

    use fgcore::logging;
    use sea_orm::ConnectionTrait;
    use sea_orm::QueryResult;
    use sea_orm_migration::MigratorTrait;
    use tauri::Listener;
    use uuid::Uuid;
    // use std::path::PathBuf;
    use crate::testutils;
    use tauri::test::mock_context;
    use tauri::test::noop_assets;
    use tauri::test::{mock_builder, MockRuntime};
    use tauri::Manager;

    #[tokio::test]
    async fn it_setups() {
        let test_id = Uuid::new_v4();
        let mut app = testutils::create_app().expect("Could not create test app");
        let config = create_config(test_id);
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
        let mut config = create_config(test_id);
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
            "SELECT COUNT(*) as count FROM user",
        );
        let result: QueryResult = connection.query_one(query).await.unwrap().unwrap();
        let count: i32 = result.try_get("", "count").unwrap();
        assert_eq!(count, 1);
    }
    #[tokio::test]
    async fn it_invokes_log() {
        let test_id = Uuid::new_v4();
        let mut app = testutils::create_app().expect("Could not create test app");
        setup::setup_async(
            &mut app,
            testutils::LOGGING_HANDLES.get().unwrap().to_owned(),
            Some(create_config(test_id)),
        )
        .await
        .unwrap();
        let (tx, rx) = tokio::sync::oneshot::channel::<String>();
        // Listen for log events.
        // The tauri_plugin_log sends events on the "tauri://log" channel.
        app.once("tauri://log", move |event| {
            let _ = tx.send(event.payload().to_string());
        });

        // Trigger a log message. (This should be caught by the tauri log plugin.)
        log::info!("This is a test log event");

        // Wait for the event to be received (with a timeout)
        let payload = time::timeout(Duration::from_secs(2), rx)
            .await
            .expect("Timed out waiting for log event")
            .expect("Failed to receive log event");

        // Option 1: Check that the payload string contains your log message.
        assert!(
            payload.contains("This is a test log event"),
            "Expected payload to contain the test log message, got: {}",
            payload
        );
    }
}

// Add to Cargo.toml:
// [dev-dependencies]
// tempfile = "3.8"
// tokio = { version = "1", features = ["full", "test-util"] }

// use tempfile::tempdir;
// use tracing::info;
//
// if !AppConfig::should_no_dotenv(app) {
//     utils::load_dotenvs(vec![
//         utils::cwd().join(".env"),
//         app.path().app_data_dir().unwrap().join("environment"),
//     ])?
// } else {
//     info!("Skipping dotenv loading");
// }
// // this hangs
// logging::reload_filter(log_handles.1)?;
// config::from_app(app).await?;

// let config = app.state::<AppConfig>();
// info!("App Config: {:?}", config);

// logging::setup_fs(&config.logs_dir, log_handles.0)?;
// info!("File tracing initialized");

// if !config.no_migrate {
//     db::migrate(&config.db_path).await?;
// }

// info!("App setup complete");

// Ok(())
