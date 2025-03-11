use std::collections::HashMap;

use crate::entity::workout::ExercisePromptStrategy;
use crate::fixtures::get_workouts_fixture;

use super::common::{MigrationTimestampExt, TableWithTimestamps};
use super::m20250115_101001_create_muscle::Muscle;
use super::m20250115_110424_create_workout as workout;

use sea_orm::{DbBackend, Statement};
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
                    .col(integer(WorkoutMuscle::Volume).not_null())
                    .col(integer(WorkoutMuscle::Priority).not_null())
                    .col(integer(WorkoutMuscle::WorkoutId).not_null())
                    // overrride split on that exercise
                    .col(integer_null(WorkoutMuscle::ExerciseSetSplit))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_workout_muscle_workout")
                            .from(WorkoutMuscle::Table, WorkoutMuscle::WorkoutId)
                            .to(workout::Workout::Table, workout::Workout::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(WorkoutMuscle::MuscleId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_workout_muscle_muscle")
                            .from(WorkoutMuscle::Table, WorkoutMuscle::MuscleId)
                            .to(Muscle::Table, Muscle::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
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

        let dbc = manager.get_connection();
        let workout_map: HashMap<String, i64> = dbc
            .query_all(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT id, code from workout",
            ))
            .await?
            .iter()
            .filter_map(|row| {
                let code = row.try_get("", "code").ok()?;
                let id = row.try_get("", "id").ok()?;
                Some((code, id))
            })
            .collect();
        let muscle_map: HashMap<String, i64> = dbc
            .query_all(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT id, code from muscle",
            ))
            .await?
            .iter()
            .filter_map(|row| {
                let code = row.try_get("", "code").ok()?;
                let id = row.try_get("", "id").ok()?;
                Some((code, id))
            })
            .collect();
        let values: Vec<Vec<Value>> = get_workouts_fixture()
            .iter()
            .flat_map(|workout| {
                workout
                    .muscles
                    .iter()
                    .map(move |muscle| (workout.code.clone(), muscle))
            })
            .map(|wm| {
                let muscle = wm.1;
                let workout_id: i64 = workout_map.get(&wm.0).unwrap().clone();
                let muscle_id: i64 = muscle_map.get(&muscle.code).unwrap().clone();
                vec![
                    Value::BigInt(Some(workout_id)),
                    Value::BigInt(Some(muscle_id)),
                    Value::Int(Some(muscle.volume.into())),
                    Value::Int(Some(muscle.priority.into())),
                    Value::Int(muscle.exercise_set_split.map(|v| v as i32)),
                ]
            })
            .collect();

        let mut insert = String::from("INSERT INTO workout_muscle (workout_id, muscle_id, volume, priority, exercise_set_split) VALUES ");
        for i in 0..values.len() {
            insert.push_str("(?, ?, ?, ?, ?)");
            if i < values.len() - 1 {
                insert.push_str(", ");
            }
        }
        let stmt = Statement::from_sql_and_values(
            DbBackend::Sqlite,
            &insert,
            values.iter().flatten().cloned().collect::<Vec<Value>>(),
        );
        // Execute the batch insert
        dbc.execute(stmt).await?;

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
