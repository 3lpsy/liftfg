use crate::fixtures::get_muscle_data_fixture;

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
                    .col(string(Muscle::Name).not_null())
                    .col(string(Muscle::Code).not_null())
                    .col(string(Muscle::LongName).not_null())
                    .add_timestamps()
                    .to_owned(),
            )
            .await?;
        self.create_timestamp_trigger(manager, Muscle::Table.to_string())
            .await?;

        let muscle_data = get_muscle_data_fixture();
        let dbc = manager.get_connection();
        for muscle in muscle_data {
            let stmt = Statement::from_sql_and_values(
                DbBackend::Sqlite,
                "INSERT INTO muscle (name, code, long_name) VALUES (?, ?, ?)",
                vec![
                    Value::String(Some(muscle.name.into())),
                    Value::String(Some(muscle.code.into())),
                    Value::String(Some(muscle.long_name.into())),
                ],
            );
            dbc.execute(stmt).await?;
        }

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
}
