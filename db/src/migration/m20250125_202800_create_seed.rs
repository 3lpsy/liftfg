use super::common::TableWithTimestamps;

use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Seed::Table)
                    .if_not_exists()
                    .col(pk_auto(Seed::Id))
                    .col(string(Seed::Name).not_null().unique_key())
                    .col(string(Seed::Status).not_null())
                    .add_timestamps()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Seed::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Seed {
    Table,
    Id,
    Name,
    Status,
}
