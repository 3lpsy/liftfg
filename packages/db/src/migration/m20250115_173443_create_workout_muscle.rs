use super::common::{MigrationTimestampExt, TableWithTimestamps};
use super::m20250115_101001_create_muscle::Muscle;
use super::m20250115_110424_create_workout as workout;

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
                    .table(WorkoutMuscle::Table)
                    .if_not_exists()
                    .col(pk_auto(WorkoutMuscle::Id))
                    .col(integer(WorkoutMuscle::Sets).not_null())
                    .col(integer(WorkoutMuscle::Priority).not_null())
                    .col(integer(WorkoutMuscle::WorkoutId).not_null())
                    .col(integer(WorkoutMuscle::CustomSetSplit))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_workout_muscle_workout") // Name of the foreign key constraint
                            .from(WorkoutMuscle::Table, WorkoutMuscle::WorkoutId) // From the workout table, profile_id column
                            .to(workout::Workout::Table, workout::Workout::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade), // To the profile table, id column
                    )
                    .col(integer(WorkoutMuscle::MuscleId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_workout_muscle_muscle") // Name of the foreign key constraint
                            .from(WorkoutMuscle::Table, WorkoutMuscle::MuscleId) // From the workout table, profile_id column
                            .to(Muscle::Table, Muscle::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade), // To the profile table, id column
                    )
                    .add_timestamps()
                    .to_owned(),
            )
            .await?;
        self.create_timestamp_trigger(manager, WorkoutMuscle::Table.to_string())
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        self.drop_timestamp_trigger(manager, WorkoutMuscle::Table.to_string())
            .await?;
        manager
            .drop_table(Table::drop().table(WorkoutMuscle::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum WorkoutMuscle {
    Table,
    Id,
    WorkoutId,
    MuscleId,
    Sets,
    Priority,
    CustomSetSplit,
}
