use super::common::{MigrationTimestampExt, TableWithTimestamps};
use super::m20250114_010338_create_program as program;

use sea_orm_migration::{prelude::*, schema::*};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TargetMuscle::Table)
                    .if_not_exists()
                    .col(pk_auto(TargetMuscle::Id))
                    .col(string(TargetMuscle::Name))
                    .add_timestamps()
                    .to_owned(),
            )
            .await?;
        self.create_timestamp_trigger(manager, TargetMuscle::Table.to_string())
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(ProgramTargetMuscle::Table)
                    .if_not_exists()
                    .col(pk_auto(ProgramTargetMuscle::Id))
                    .col(integer(ProgramTargetMuscle::Sets).not_null())
                    .col(integer(ProgramTargetMuscle::ProgramId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_program_target_muscle_program") // Name of the foreign key constraint
                            .from(ProgramTargetMuscle::Table, ProgramTargetMuscle::ProgramId) // From the program table, profile_id column
                            .to(program::Program::Table, program::Program::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade), // To the profile table, id column
                    )
                    .col(integer(ProgramTargetMuscle::TargetMuscleId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_program_target_muscle_target_muscle") // Name of the foreign key constraint
                            .from(
                                ProgramTargetMuscle::Table,
                                ProgramTargetMuscle::TargetMuscleId,
                            ) // From the program table, profile_id column
                            .to(TargetMuscle::Table, TargetMuscle::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade), // To the profile table, id column
                    )
                    .index(
                        Index::create()
                            .name("idx_program_target_muscle_unique")
                            .table(ProgramTargetMuscle::Table)
                            .col(ProgramTargetMuscle::TargetMuscleId)
                            .col(ProgramTargetMuscle::ProgramId)
                            .unique(),
                    )
                    .add_timestamps()
                    .to_owned(),
            )
            .await?;
        self.create_timestamp_trigger(manager, ProgramTargetMuscle::Table.to_string())
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        self.drop_timestamp_trigger(manager, TargetMuscle::Table.to_string())
            .await?;
        self.drop_timestamp_trigger(manager, ProgramTargetMuscle::Table.to_string())
            .await?;
        manager
            .drop_table(Table::drop().table(ProgramTargetMuscle::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(TargetMuscle::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TargetMuscle {
    Table,
    Id,
    Name,
}
#[derive(DeriveIden)]
enum ProgramTargetMuscle {
    Table,
    Id,
    ProgramId,
    TargetMuscleId,
    Sets,
}
