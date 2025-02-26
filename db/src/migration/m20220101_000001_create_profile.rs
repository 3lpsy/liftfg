use super::common::{MigrationTimestampExt, TableWithTimestamps};
use sea_orm_migration::{prelude::*, schema::*}; // Import the trait

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Profile::Table)
                    .if_not_exists()
                    .col(pk_auto(Profile::Id))
                    .col(boolean(Profile::IsDefault).not_null().default(false))
                    .col(string(Profile::Name).not_null().unique_key())
                    .add_timestamps()
                    .to_owned(),
            )
            .await?;
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                        CREATE UNIQUE INDEX unique_default_profile
                        ON profile (is_default)
                        WHERE is_default = TRUE;
                        "#,
            )
            .await?;
        self.create_timestamp_trigger(manager, Profile::Table.to_string())
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        self.drop_timestamp_trigger(manager, Profile::Table.to_string())
            .await?;
        manager
            .get_connection()
            .execute_unprepared("DROP INDEX IF EXISTS unique_default_profile;")
            .await?;
        manager
            .drop_table(Table::drop().table(Profile::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Profile {
    Table,
    Id,
    IsDefault,
    Name,
}
