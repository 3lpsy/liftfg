use super::common::TableWithTimestamps;
use super::m20220101_000001_create_profile as profile;

use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Program::Table)
                    .if_not_exists()
                    .col(pk_auto(Program::Id))
                    .col(string(Program::Name).not_null().unique_key())
                    .col(integer(Program::ProfileId).not_null()) // Add the foreign key column
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_program_profile") // Name of the foreign key constraint
                            .from(Program::Table, Program::ProfileId)
                            .to(profile::Profile::Table, profile::Profile::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .add_timestamps()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Program::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Program {
    Table,
    Id,
    Name,
    ProfileId,
}
