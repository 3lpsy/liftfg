#[cfg(test)]
mod tests {
    use crate::config::AppConfig;
    use crate::handlers;
    use crate::setup;

    use db::db::get_dbc;
    use db::migration;

    use fgcore::logging;
    use sea_orm::ConnectionTrait;
    use sea_orm::QueryResult;
    use sea_orm_migration::MigratorTrait;
    use std::path::PathBuf;
    use tauri::test::{mock_builder, MockRuntime};
    use tauri::Manager;
    use tempfile::tempdir;

    async fn setup_test_app() -> (tauri::App<MockRuntime>, PathBuf) {
        let temp_dir = tempdir().unwrap();
        let app_data_dir = temp_dir.path().to_path_buf();
        let log_handles = logging::setup().unwrap();
        tracing::debug!("Before build mock");

        let app = mock_builder()
            .setup(|app| Ok(setup::setup(app, log_handles)?))
            .invoke_handler(handlers::generate())
            .build(tauri::generate_context!("../shell/tauri.conf.json"))
            .unwrap();
        (app, app_data_dir)
    }

    #[tokio::test]
    async fn it_setups() {
        let (app, app_data_dir) = setup_test_app().await;
        tracing::debug!("Before app access mock");

        let config = app.state::<AppConfig>();
        assert!(config.db_path.exists());
        assert!(config.logs_dir.exists());
        let db_path = app_data_dir.join("test.db");
        assert!(db_path.exists());
        let connection = get_dbc(&db_path).await.unwrap();
        let query = sea_orm::Statement::from_string(
            connection.get_database_backend(),
            "SELECT COUNT(*) as count FROM seaql_migrations",
        );
        let result: QueryResult = connection.query_one(query).await.unwrap().unwrap();
        let count: i32 = result.try_get("", "count").unwrap();
        assert_eq!(count, migration::Migrator::migrations().len() as i32);
    }

    // #[tokio::test]
    // async fn test_env_loading() {
    //     let temp_dir = tempdir().unwrap();
    //     let env_path = temp_dir.path().join(".env");

    //     std::fs::write(&env_path, "TEST_VAR=test_value").unwrap();

    //     let (app, _) = setup_test_app().await;
    //     assert_eq!(std::env::var("TEST_VAR").unwrap(), "test_value");
    // }

    // #[tokio::test]
    // async fn test_logging_setup() {
    //     let (app, app_data_dir) = setup_test_app().await;

    //     info!("Test log message");

    //     let log_file = app_data_dir.join("logs").join("app.log");
    //     assert!(log_file.exists());

    //     let log_content = std::fs::read_to_string(log_file).unwrap();
    //     assert!(log_content.contains("Test log message"));
    // }

    // #[test]
    // fn test_plugin_setup() {
    //     let builder = tauri::Builder::default();
    //     let configured_builder = plugins::setup(builder);

    //     // Add assertions for plugin configuration
    //     // This will depend on your plugin setup
    // }
}

// Add to Cargo.toml:
// [dev-dependencies]
// tempfile = "3.8"
// tokio = { version = "1", features = ["full", "test-util"] }
