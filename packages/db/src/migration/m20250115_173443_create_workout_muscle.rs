use crate::entity::workout::ExercisePromptStrategy;
use crate::fixtures::get_workout_data_fixture;

use super::common::{MigrationTimestampExt, TableWithTimestamps};
use super::m20250115_101001_create_muscle::Muscle;
use super::m20250115_110424_create_workout as workout;

use sea_orm::{DbBackend, Statement};
use sea_orm_migration::{prelude::*, schema::*};
use tracing::warn;
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
                    .col(integer(WorkoutMuscle::Volume).not_null())
                    .col(integer(WorkoutMuscle::Priority).not_null())
                    .col(integer(WorkoutMuscle::WorkoutId).not_null())
                    // overrride split on that exercise
                    .col(integer_null(WorkoutMuscle::ExerciseSetSplit))
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
                    .col(
                        string_null(WorkoutMuscle::ExercisePromptStrategy)
                            .default(ExercisePromptStrategy::CommonCompound),
                    )
                    .add_timestamps()
                    .to_owned(),
            )
            .await?;
        self.create_timestamp_trigger(manager, WorkoutMuscle::Table.to_string())
            .await?;

        let workouts_data = get_workout_data_fixture();
        let dbc = manager.get_connection();

        for workout in workouts_data {
            // Get workout ID
            let workout_id: i64 = dbc
                .query_one(Statement::from_sql_and_values(
                    DbBackend::Sqlite,
                    "SELECT id FROM workout WHERE code = ?",
                    vec![Value::String(Some(workout.code.clone().into()))],
                ))
                .await?
                .map(|row| row.try_get::<i64>("", "id").unwrap())
                .expect("Workout not found");

            for muscle in workout.muscles {
                // Get muscle ID
                let muscle_id: i64 = dbc
                    .query_one(Statement::from_sql_and_values(
                        DbBackend::Sqlite,
                        "SELECT id FROM muscle WHERE code = ?",
                        vec![Value::String(Some(muscle.code.clone().into()))],
                    ))
                    .await?
                    .map(|row| row.try_get::<i64>("", "id").unwrap())
                    .expect("Muscle not found");

                // Insert into pivot table
                let stmt = Statement::from_sql_and_values(
                    DbBackend::Sqlite,
                    "INSERT INTO workout_muscle (workout_id, muscle_id, volume, priority, exercise_set_split) VALUES (?, ?, ?, ?, ?)",
                    vec![
                        Value::BigInt(Some(workout_id)),
                        Value::BigInt(Some(muscle_id)),
                        Value::Int(Some(muscle.volume.into())),
                        Value::Int(Some(muscle.priority.into())),
                        Value::Int(muscle.exercise_set_split.map(|v| v as i32))
                    ],
                );
                dbc.execute(stmt).await?;
            }
        }

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
    Volume,
    Priority,
    ExerciseSetSplit,
    ExercisePromptStrategy,
}
