use super::common::{MigrationTimestampExt, TableWithTimestamps};
use super::m20250115_101001_create_muscle::Muscle;
use super::m20250115_110424_create_program as program;

use sea_orm_migration::{prelude::*, schema::*};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // no index so can be added multiple times
        manager
            .create_table(
                Table::create()
                    .table(ProgramMuscle::Table)
                    .if_not_exists()
                    .col(pk_auto(ProgramMuscle::Id))
                    .col(integer(ProgramMuscle::Sets).not_null())
                    .col(integer(ProgramMuscle::Priority).not_null())
                    .col(integer(ProgramMuscle::ProgramId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_program_muscle_program") // Name of the foreign key constraint
                            .from(ProgramMuscle::Table, ProgramMuscle::ProgramId) // From the program table, profile_id column
                            .to(program::Program::Table, program::Program::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade), // To the profile table, id column
                    )
                    .col(integer(ProgramMuscle::MuscleId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_program_muscle_muscle") // Name of the foreign key constraint
                            .from(ProgramMuscle::Table, ProgramMuscle::MuscleId) // From the program table, profile_id column
                            .to(Muscle::Table, Muscle::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade), // To the profile table, id column
                    )
                    .add_timestamps()
                    .to_owned(),
            )
            .await?;
        self.create_timestamp_trigger(manager, ProgramMuscle::Table.to_string())
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        self.drop_timestamp_trigger(manager, ProgramMuscle::Table.to_string())
            .await?;
        manager
            .drop_table(Table::drop().table(ProgramMuscle::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ProgramMuscle {
    Table,
    Id,
    ProgramId,
    MuscleId,
    Sets,
    Priority,
}
