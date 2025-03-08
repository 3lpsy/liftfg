use crate::entity::workout::{ExcerciseSplitStrategy, ExercisePromptStrategy, MuscleOrderStrategy};
use crate::fixtures::get_workout_data_fixture;

use super::common::{MigrationTimestampExt, TableWithTimestamps};
use super::m20220101_000001_create_profile as profile;

use sea_orm::{DatabaseBackend, DbBackend, Statement};
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
                    .col(string(Workout::Name).not_null())
                    .col(string(Workout::Code).unique_key().not_null())
                    .col(
                        string(Workout::MuscleOrderStrategy)
                            .not_null()
                            .default(MuscleOrderStrategy::Deterministic),
                    )
                    .col(
                        string(Workout::ExcerciseSplitStrategy)
                            .not_null()
                            .default(ExcerciseSplitStrategy::Simple),
                    )
                    .col(
                        string(Workout::ExercisePromptStrategy)
                            .not_null()
                            .default(ExercisePromptStrategy::CommonCompound),
                    )
                    .col(integer(Workout::ExerciseSetSplit).not_null().default(3))
                    .add_timestamps()
                    .to_owned(),
            )
            .await?;
        self.create_timestamp_trigger(manager, Workout::Table.to_string())
            .await?;

        let workouts_data = get_workout_data_fixture();
        let dbc = manager.get_connection();
        for workout in workouts_data {
            let stmt = Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "INSERT INTO workout (name, code) VALUES (?, ?)",
                vec![
                    Value::String(Some(workout.name.into())),
                    Value::String(Some(workout.code.into())),
                ],
            );
            dbc.execute(stmt).await?;
        }

        manager
            .create_table(
                Table::create()
                    .table(ProfileWorkout::Table)
                    .if_not_exists()
                    .col(pk_auto(ProfileWorkout::Id))
                    .col(integer(ProfileWorkout::ProfileId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_profile_workout_profile") // Name of the foreign key constraint
                            .from(ProfileWorkout::Table, ProfileWorkout::ProfileId) // From the workout table, profile_id column
                            .to(profile::Profile::Table, profile::Profile::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(ProfileWorkout::WorkoutId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_profile_workout_workout")
                            .from(ProfileWorkout::Table, ProfileWorkout::WorkoutId)
                            .to(Workout::Table, Workout::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .add_timestamps()
                    .index(
                        Index::create()
                            .name("idx_profile_workout_unique")
                            .table(ProfileWorkout::Table)
                            .col(ProfileWorkout::ProfileId)
                            .col(ProfileWorkout::WorkoutId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;
        self.create_timestamp_trigger(manager, ProfileWorkout::Table.to_string())
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        self.drop_timestamp_trigger(manager, ProfileWorkout::Table.to_string())
            .await?;
        self.drop_timestamp_trigger(manager, Workout::Table.to_string())
            .await?;
        manager
            .drop_table(Table::drop().table(ProfileWorkout::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Workout::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Workout {
    Table,
    Id,
    Name,
    Code,
    ProfileId,
    MuscleOrderStrategy,
    ExcerciseSplitStrategy,
    ExercisePromptStrategy,
    ExerciseSetSplit,
}
#[derive(DeriveIden)]
enum ProfileWorkout {
    Table,
    Id,
    ProfileId,
    WorkoutId,
}
