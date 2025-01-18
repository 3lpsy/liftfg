use crate::migration;
use anyhow::{Error, Result};
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use std::env;
use std::{fs::OpenOptions, path::Path};
use tracing::info;

fn touch(path: &String) -> Result<()> {
    if !Path::new(path).exists() {
        OpenOptions::new().create(true).write(true).open(path)?;
    }
    Ok(())
}

pub async fn migrate(db_path: Option<&String>) -> Result<()> {
    let db = get_db(db_path).await?;
    let r = migration::Migrator::up(&db, None).await?;
    info!("Database migrated");
    return Ok(r);
}
pub async fn rollback(db_path: Option<&String>) -> Result<()> {
    let db = get_db(db_path).await?;
    let r = migration::Migrator::down(&db, Some(1)).await?;
    info!("Database rollbacked");
    return Ok(r);
}

pub async fn get_db(db_path: Option<&String>) -> Result<DatabaseConnection> {
    let mut db_url: String;
    if db_path.is_some() {
        db_url = db_path.unwrap().to_owned();
    } else if env::var("DATABASE_PATH").is_ok() {
        db_url = env::var("DATABASE_PATH").unwrap();
    } else {
        return Err(Error::msg(
            "No database path provided in config or environment.",
        ));
    }
    touch(&db_url)?;
    db_url = format!("sqlite://{}", db_url);
    info!("Connecting: {}", db_url);
    Ok(Database::connect(db_url).await?)
}
