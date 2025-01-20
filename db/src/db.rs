use crate::migration;
use anyhow::Result;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use std::fs::OpenOptions;
use std::path::PathBuf;
use tracing::info;

fn touch(path: &PathBuf) -> Result<()> {
    if !path.exists() {
        OpenOptions::new().create(true).write(true).open(path)?;
    }
    Ok(())
}

pub async fn migrate(db_path: &PathBuf) -> Result<()> {
    let db = get_db(db_path).await?;
    let r = migration::Migrator::up(&db, None).await?;
    info!("Database migrated");
    return Ok(r);
}
pub async fn rollback(db_path: &PathBuf) -> Result<()> {
    let db = get_db(db_path).await?;
    let r = migration::Migrator::down(&db, Some(1)).await?;
    info!("Database rollbacked");
    return Ok(r);
}

pub async fn get_db(db_path: &PathBuf) -> Result<DatabaseConnection> {
    touch(db_path)?;
    let db_url = format!("sqlite://{}", db_path.to_string_lossy().to_string());
    info!("Connecting: {}", db_url);
    Ok(Database::connect(db_url).await?)
}
