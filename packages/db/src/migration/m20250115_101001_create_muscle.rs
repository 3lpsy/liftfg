use crate::fixtures::get_muscles_fixture;

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
                    .table(Muscle::Table)
                    .if_not_exists()
                    .col(pk_auto(Muscle::Id))
                    .col(string(Muscle::Name).not_null().unique_key())
                    .col(string(Muscle::Code).not_null().unique_key())
                    .col(string(Muscle::LongName).not_null().unique_key())
                    .col(integer(Muscle::SizeScore).default(3).not_null())
                    .add_timestamps()
                    .to_owned(),
            )
            .await?;
        self.create_timestamp_trigger(manager, Muscle::Table.to_string())
            .await?;

        let dbc = manager.get_connection();

        let values: Vec<Vec<Value>> = get_muscles_fixture()
            .iter()
            .map(|item| {
                vec![
                    Value::String(Some(item.name.clone().into())),
                    Value::String(Some(item.code.clone().into())),
                    Value::String(Some(item.long_name.clone().into())),
                    Value::Int(Some(item.size_score.clone().into())),
                ]
            })
            .collect();
        let mut insert =
            String::from("INSERT INTO muscle (name, code, long_name, size_score) VALUES ");
        for i in 0..values.len() {
            insert.push_str("(?, ?, ?, ?)");
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
        self.drop_timestamp_trigger(manager, Muscle::Table.to_string())
            .await?;
        manager
            .drop_table(Table::drop().table(Muscle::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Muscle {
    Table,
    Id,
    Code,
    Name,
    LongName,
    SizeScore,
}
