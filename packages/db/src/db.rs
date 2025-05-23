#[cfg(feature = "db")]
// only db
use crate::migration;
use anyhow::Result;
use fgutils::touch;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use std::path::PathBuf;
use tracing::{debug, info};

pub async fn migrate(db_path: &PathBuf) -> Result<()> {
    let db = get_dbc(db_path).await?;
    let r = migration::Migrator::up(&db, None).await?;
    info!("Database migrated");
    return Ok(r);
}

pub async fn rollback(db_path: &PathBuf) -> Result<()> {
    let db = get_dbc(db_path).await?;
    let r = migration::Migrator::down(&db, Some(1)).await?;
    info!("Database rollbacked");
    return Ok(r);
}

pub async fn get_dbc(db_path: &PathBuf) -> Result<DatabaseConnection> {
    // TODO: Okay, the parent directory might not exist. Probably just create
    touch(db_path)?;
    let db_url = get_db_url(db_path);
    debug!("Connecting: {}", db_url);
    let options = ConnectOptions::new(db_url);
    // options
    // .max_connections(10)
    // .min_connections(5)
    // .connect_timeout(Duration::from_secs(8));
    // .idle_timeout(Duration::from_secs(8));

    Ok(Database::connect(options).await?)
}

pub fn get_db_url(db_path: &PathBuf) -> String {
    format!("sqlite://{}", db_path.to_string_lossy().to_string())
}

// pub async fn get_dbc_sqlx(db_path: &PathBuf) -> Result<sqlx::Pool<sqlx::Sqlite>> {
//     touch(db_path)?;
//     let db_url = get_db_url(db_path);
//     let pool_options = sqlx::pool::PoolOptions::<sqlx::Sqlite>::new().max_connections(1);
//     pool_options.connect(&db_url).await.map_err(Into::into)
// }
