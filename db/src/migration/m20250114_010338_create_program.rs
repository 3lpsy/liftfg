use super::common::{MigrationTimestampExt, TableWithTimestamps};
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
                    .add_timestamps()
                    .to_owned(),
            )
            .await?;
        self.create_timestamp_trigger(manager, Program::Table.to_string())
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ProfileProgram::Table)
                    .if_not_exists()
                    .col(pk_auto(ProfileProgram::Id))
                    .col(integer(ProfileProgram::ProfileId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_profile_program_profile") // Name of the foreign key constraint
                            .from(ProfileProgram::Table, ProfileProgram::ProfileId) // From the program table, profile_id column
                            .to(profile::Profile::Table, profile::Profile::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(ProfileProgram::ProgramId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_profile_program_program")
                            .from(ProfileProgram::Table, ProfileProgram::ProgramId)
                            .to(Program::Table, Program::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .add_timestamps()
                    .index(
                        Index::create()
                            .name("idx_profile_program_unique")
                            .table(ProfileProgram::Table)
                            .col(ProfileProgram::ProfileId)
                            .col(ProfileProgram::ProgramId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;
        self.create_timestamp_trigger(manager, ProfileProgram::Table.to_string())
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        self.drop_timestamp_trigger(manager, ProfileProgram::Table.to_string())
            .await?;
        self.drop_timestamp_trigger(manager, Program::Table.to_string())
            .await?;
        manager
            .drop_table(Table::drop().table(ProfileProgram::Table).to_owned())
            .await?;
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
#[derive(DeriveIden)]
enum ProfileProgram {
    Table,
    Id,
    ProfileId,
    ProgramId,
}
