use super::common::TableWithTimestamps;
use super::m20250115_175632_create_exercise::Exercise;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Workout::Table)
                    .if_not_exists()
                    .col(pk_auto(Workout::Id))
                    .add_timestamps()
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Set::Table)
                    .if_not_exists()
                    .col(pk_auto(Set::Id))
                    .col(integer(Set::Reps))
                    .col(integer(Set::Weight))
                    .col(integer(Set::WorkoutId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_set_workout")
                            .from(Set::Table, Set::WorkoutId)
                            .to(Workout::Table, Workout::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(Set::ExerciseId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_set_exercise")
                            .from(Set::Table, Set::ExerciseId)
                            .to(Exercise::Table, Exercise::Id)
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
            .drop_table(Table::drop().table(Workout::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Set::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Workout {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Set {
    Table,
    Id,
    Reps,
    Weight,
    ExerciseId,
    WorkoutId,
}
