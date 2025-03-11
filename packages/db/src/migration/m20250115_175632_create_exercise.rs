use crate::fixtures::get_exercises_fixture;

use super::common::{MigrationTimestampExt, TableWithTimestamps};
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
        let values: Vec<Vec<Value>> = get_exercises_fixture()
            .iter()
            .map(|item| {
                vec![
                    Value::String(Some(item.name.clone().into())),
                    Value::String(Some(item.code.clone().into())),
                    Value::String(Some(item.movement_code.clone().into())),
                    Value::String(Some(item.equipment_type.clone().into())),
                    Value::Int(Some(item.fatigue_score.clone().into())),
                ]
            })
            .collect();
        let mut insert = String::from("INSERT INTO exercise (name, code, movement_code, equipment_type, fatigue_score) VALUES ");
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
