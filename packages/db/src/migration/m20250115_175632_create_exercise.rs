use super::{
    common::{MigrationTimestampExt, TableWithTimestamps},
    m20250115_101001_create_muscle::Muscle,
};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Exercise::Table)
                    .if_not_exists()
                    .col(pk_auto(Exercise::Id))
                    .col(string(Exercise::Name).not_null())
                    .col(string(Exercise::EquipmentType).not_null().default("OTHER"))
                    .add_timestamps()
                    .to_owned(),
            )
            .await?;
        self.create_timestamp_trigger(manager, Exercise::Table.to_string())
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(ExerciseMuscle::Table)
                    .if_not_exists()
                    .col(pk_auto(ExerciseMuscle::Id))
                    .col(integer(ExerciseMuscle::Effectiveness).not_null())
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
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        self.drop_timestamp_trigger(manager, ExerciseMuscle::Table.to_string())
            .await?;
        self.drop_timestamp_trigger(manager, Exercise::Table.to_string())
            .await?;
        manager
            .drop_table(Table::drop().table(ExerciseMuscle::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Exercise::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Exercise {
    Table,
    Id,
    Name,
    EquipmentType,
}
#[derive(DeriveIden)]
enum ExerciseMuscle {
    Table,
    Id,
    MuscleId,
    ExerciseId,
    Effectiveness,
}
