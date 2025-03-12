use crate::fixtures::get_muscles_fixture;

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
        let columns: Vec<Alias> = [
            Muscle::Name.to_string(),
            Muscle::Code.to_string(),
            Muscle::LongName.to_string(),
            Muscle::SizeScore.to_string(),
        ]
        .into_iter()
        .map(Alias::new)
        .collect();
        let mut insert = Query::insert();
        insert.into_table(Muscle::Table).columns(columns);
        get_muscles_fixture().iter().for_each(|item| {
            insert.values_panic([
                item.name.clone().into(),
                item.code.clone().into(),
                item.long_name.clone().into(),
                item.size_score.clone().into(),
            ]);
        });
        let builder = dbc.get_database_backend();
        dbc.execute(builder.build(&insert)).await?;

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
