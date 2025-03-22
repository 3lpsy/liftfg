use super::common::{MigrationTimestampExt, TableWithTimestamps};
use super::m20220101_000001_create_profile::Profile;
use super::m20250115_101000_create_gym::Gym;
use super::m20250115_110424_create_workout::Workout;
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
                    .table(Session::Table)
                    .if_not_exists()
                    .col(pk_auto(Session::Id))
                    // do not cascade
                    .col(integer_null(Session::WorkoutId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_set_session")
                            .from(Session::Table, Session::WorkoutId)
                            .to(Workout::Table, Workout::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::SetNull),
                    )
                    // do not cascade
                    .col(integer_null(Session::ProfileId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_set_session")
                            .from(Session::Table, Session::ProfileId)
                            .to(Profile::Table, Profile::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::SetNull),
                    )
                    // do not cascade
                    .col(integer_null(Session::GymId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_set_session")
                            .from(Session::Table, Session::GymId)
                            .to(Gym::Table, Gym::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::SetNull),
                    )
                    .add_timestamps()
                    .to_owned(),
            )
            .await?;
        self.create_timestamp_trigger(manager, Session::Table.to_string())
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Set::Table)
                    .if_not_exists()
                    .col(pk_auto(Set::Id))
                    .col(integer(Set::Reps))
                    .col(integer(Set::Weight))
                    .col(integer(Set::SessionId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_set_session")
                            .from(Set::Table, Set::SessionId)
                            .to(Session::Table, Session::Id)
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
            .await?;
        self.create_timestamp_trigger(manager, Set::Table.to_string())
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        self.drop_timestamp_trigger(manager, Set::Table.to_string())
            .await?;
        self.drop_timestamp_trigger(manager, Session::Table.to_string())
            .await?;
        manager
            .drop_table(Table::drop().table(Session::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Set::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Session {
    Table,
    Id,
    WorkoutId,
    ProfileId,
    GymId,
}

#[derive(DeriveIden)]
enum Set {
    Table,
    Id,
    Reps,
    Weight,
    ExerciseId,
    SessionId,
}
