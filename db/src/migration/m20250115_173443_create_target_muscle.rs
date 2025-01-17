use super::common::TableWithTimestamps;
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
        manager
            .create_table(
                Table::create()
                    .table(ProgramTargetMuscle::Table)
                    .if_not_exists()
                    .col(pk_auto(ProgramTargetMuscle::Id))
                    .col(integer(ProgramTargetMuscle::Sets))
                    .col(integer(ProgramTargetMuscle::ProgramId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_program_target_muscle_program") // Name of the foreign key constraint
                            .from(ProgramTargetMuscle::Table, ProgramTargetMuscle::ProgramId) // From the program table, user_id column
                            .to(program::Program::Table, program::Program::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade), // To the user table, id column
                    )
                    .col(integer(ProgramTargetMuscle::TargetMuscleId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_program_target_muscle_target_muscle") // Name of the foreign key constraint
                            .from(
                                ProgramTargetMuscle::Table,
                                ProgramTargetMuscle::TargetMuscleId,
                            ) // From the program table, user_id column
                            .to(TargetMuscle::Table, TargetMuscle::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade), // To the user table, id column
                    )
                    .add_timestamps()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
