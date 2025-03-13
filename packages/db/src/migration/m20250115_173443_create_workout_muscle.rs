use std::collections::HashMap;

use crate::data::enums::ExercisePromptStrategy;
use crate::fixtures::get_workouts_fixture;

use super::common::{MigrationTimestampExt, TableWithTimestamps};
use super::m20250115_101001_create_muscle::Muscle;
use super::m20250115_110424_create_workout::{self as workout};

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
            .map(|row| {
                let code = row.try_get("", "code").unwrap();
                let id = row.try_get("", "id").unwrap();
                (code, id)
            })
            .collect();
        let muscle_map: HashMap<String, i64> = dbc
            .query_all(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT id, code from muscle",
            ))
            .await?
            .iter()
            .map(|row| {
                let code = row.try_get("", "code").unwrap();
                let id = row.try_get("", "id").unwrap();
                (code, id)
            })
            .collect();
        let columns: Vec<Alias> = [
            WorkoutMuscle::WorkoutId.to_string(),
            WorkoutMuscle::MuscleId.to_string(),
            WorkoutMuscle::Volume.to_string(),
            WorkoutMuscle::Priority.to_string(),
            WorkoutMuscle::ExerciseSetSplit.to_string(),
        ]
        .into_iter()
        .map(Alias::new)
        .collect();

        let mut insert = Query::insert();
        insert.into_table(WorkoutMuscle::Table).columns(columns);

        get_workouts_fixture()
            .iter()
            .flat_map(|workout| {
                workout
                    .muscles
                    .iter()
                    .map(move |muscle| (workout.code.clone(), muscle))
            })
            .for_each(|wm| {
                let muscle = wm.1;
                let exercise_set_split = match muscle.exercise_set_split {
                    Some(val) => {
                        sea_query::SimpleExpr::Value(sea_query::Value::Int(Some(val as i32)))
                    }
                    None => sea_query::SimpleExpr::Value(sea_query::Value::Int(None)),
                };
                insert.values_panic([
                    (*workout_map.get(&wm.0).unwrap()).into(),
                    (*muscle_map.get(&muscle.code).unwrap()).into(),
                    muscle.volume.into(),
                    muscle.priority.into(),
                    exercise_set_split,
                ]);
            });
        let builder = dbc.get_database_backend();
        dbc.execute(builder.build(&insert)).await?;
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
