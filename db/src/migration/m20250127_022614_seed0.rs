use crate::entity::seed::{self, Status};
use sea_orm::ActiveValue;
use sea_orm::{ActiveModelTrait, TransactionTrait};
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

// because we're in a migration, this will be tracked, seed is basically a worthless table and should be removed
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let transaction = db.begin().await?;
        seed::ActiveModel {
            name: ActiveValue::Set("INIT".to_owned()),
            status: ActiveValue::Set(Status::Success),
            ..Default::default()
        }
        .insert(&transaction)
        .await?;
        transaction.commit().await?;
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

// #[derive(DeriveIden)]
// enum Post {
//     Table,
//     Id,
//     Title,
//     Text,
// }
