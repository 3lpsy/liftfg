use crate::fixtures::get_exercises_fixture;

use super::common::{MigrationTimestampExt, TableWithTimestamps};
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
                    .col(string(Exercise::MovementCode).not_null())
                    .col(string(Exercise::Code).not_null())
                    .col(string(Exercise::EquipmentType).not_null().default("OTHER"))
                    .col(integer(Exercise::FatigueScore).default(3))
                    .add_timestamps()
                    .to_owned(),
            )
            .await?;
        self.create_timestamp_trigger(manager, Exercise::Table.to_string())
            .await?;

        let dbc = manager.get_connection();
        let columns: Vec<Alias> = [
            Exercise::Name.to_string(),
            Exercise::Code.to_string(),
            Exercise::MovementCode.to_string(),
            Exercise::EquipmentType.to_string(),
            Exercise::FatigueScore.to_string(),
        ]
        .into_iter()
        .map(Alias::new)
        .collect();
        let mut insert = Query::insert();
        insert.into_table(Exercise::Table).columns(columns);
        get_exercises_fixture().iter().for_each(|item| {
            insert.values_panic([
                item.name.clone().into(),
                item.code.clone().into(),
                item.movement_code.clone().into(),
                item.equipment_type.clone().into(),
                item.fatigue_score.into(),
            ]);
        });
        let builder = dbc.get_database_backend();
        dbc.execute(builder.build(&insert)).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        self.drop_timestamp_trigger(manager, Exercise::Table.to_string())
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
    Code,
    MovementCode,
    EquipmentType,
    FatigueScore,
}
