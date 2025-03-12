use std::collections::HashMap;

use crate::fixtures::get_exercises_fixture;

use super::{
    common::{MigrationTimestampExt, TableWithTimestamps},
    m20250115_101001_create_muscle::Muscle,
    m20250115_175632_create_exercise::Exercise,
};
use sea_orm::{DbBackend, Statement};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ExerciseMuscle::Table)
                    .if_not_exists()
                    .col(pk_auto(ExerciseMuscle::Id))
                    .col(integer(ExerciseMuscle::EffectScore).not_null())
                    .col(integer(ExerciseMuscle::ExerciseId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_exercise_muscle_exercise")
                            .from(ExerciseMuscle::Table, ExerciseMuscle::ExerciseId)
                            .to(Exercise::Table, Exercise::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(ExerciseMuscle::MuscleId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_exercise_muscle_muscle")
                            .from(ExerciseMuscle::Table, ExerciseMuscle::MuscleId)
                            .to(Muscle::Table, Muscle::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("idx_exercise_muscle")
                            .table(ExerciseMuscle::Table)
                            .col(ExerciseMuscle::MuscleId)
                            .col(ExerciseMuscle::ExerciseId)
                            .unique(),
                    )
                    .add_timestamps()
                    .to_owned(),
            )
            .await?;
        self.create_timestamp_trigger(manager, ExerciseMuscle::Table.to_string())
            .await?;
        let dbc = manager.get_connection();
        let excercise_map: HashMap<String, i64> = dbc
            .query_all(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT id, code from exercise",
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
            ExerciseMuscle::ExerciseId.to_string(),
            ExerciseMuscle::MuscleId.to_string(),
            ExerciseMuscle::EffectScore.to_string(),
        ]
        .into_iter()
        .map(Alias::new)
        .collect();

        let mut insert = Query::insert();
        insert.into_table(ExerciseMuscle::Table).columns(columns);

        get_exercises_fixture()
            .iter()
            .flat_map(|ex| {
                ex.muscles
                    .iter()
                    .map(move |muscle| (ex.code.clone(), muscle))
            })
            .for_each(|wm| {
                insert.values_panic([
                    (*excercise_map.get(&wm.0).unwrap()).into(),
                    (*muscle_map.get(&wm.1.code).unwrap()).into(),
                    wm.1.effectiveness.into(),
                ]);
            });
        let builder = dbc.get_database_backend();
        dbc.execute(builder.build(&insert)).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        self.drop_timestamp_trigger(manager, ExerciseMuscle::Table.to_string())
            .await?;
        manager
            .drop_table(Table::drop().table(ExerciseMuscle::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ExerciseMuscle {
    Table,
    Id,
    MuscleId,
    ExerciseId,
    EffectScore,
}
