use crate::entity::seed::{self, Status};
use sea_orm::{ActiveModelTrait, TransactionTrait};
use sea_orm::{ActiveValue, TransactionError};
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

// because we're in a migration, this will be tracked, seed is basically a worthless table and should be removed
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let dbc = manager.get_connection();
        dbc.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                seed::ActiveModel {
                    name: ActiveValue::Set("INIT".to_owned()),
                    status: ActiveValue::Set(Status::Success),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                Ok(())
            })
        })
        .await
        .map_err(|e| match e {
            TransactionError::Connection(e) => e,
            TransactionError::Transaction(e) => e,
        })?;

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
