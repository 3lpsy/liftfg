#[cfg(test)]
mod tests {
    use crate::config::AppConfig;
    use crate::handlers;
    use crate::plugins;
    use crate::setup;

    use db::db::get_dbc;
    use db::migration;

    use fgcore::logging;
    use sea_orm::ConnectionTrait;
    use sea_orm::QueryResult;
    use sea_orm_migration::MigratorTrait;
    use std::path::PathBuf;
    use tauri::test::mock_context;
    use tauri::test::noop_assets;
    use tauri::test::{mock_builder, MockRuntime};
    use tauri::Manager;
    use tempfile::tempdir;
    use tracing::info;

    async fn create_test_app() -> tauri::App<MockRuntime> {
        std::env::set_var("APP_ENV", "test");
        std::env::set_var("DATABASE_PATH", "test.db");
        std::env::set_var("SKIP_DOTENV", "true");
        let mut builder = mock_builder();
        builder = plugins::setup(builder);
        builder
            .setup(|_app| Ok(()))
            .invoke_handler(handlers::generate())
            .build(mock_context(noop_assets()))
            .unwrap()
    }

    #[tokio::test]
    async fn it_setups() {
        let log_handles = logging::setup().unwrap();
        let mut app = create_test_app().await;
        // before setup is called, env needs to be setup

        setup::setup_async(&mut app, log_handles).await.unwrap();
        let config = app.state::<AppConfig>();
        assert!(config.db_path.exists());
        assert!(config.logs_dir.exists());
        assert!(config.db_path.exists());
        let connection = get_dbc(&config.db_path).await.unwrap();
        let query = sea_orm::Statement::from_string(
            connection.get_database_backend(),
            "SELECT COUNT(*) as count FROM seaql_migrations",
        );
        let result: QueryResult = connection.query_one(query).await.unwrap().unwrap();
        let count: i32 = result.try_get("", "count").unwrap();
        assert_eq!(count, migration::Migrator::migrations().len() as i32);
    }
}

// Add to Cargo.toml:
// [dev-dependencies]
// tempfile = "3.8"
// tokio = { version = "1", features = ["full", "test-util"] }
