use super::common::{MigrationTimestampExt, TableWithTimestamps};
use super::m20220101_000001_create_profile as profile;

use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Gym::Table)
                    .if_not_exists()
                    .col(pk_auto(Gym::Id))
                    .col(string(Gym::Name).unique_key())
                    .add_timestamps()
                    .to_owned(),
            )
            .await?;
        self.create_timestamp_trigger(manager, Gym::Table.to_string())
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(GymProfile::Table)
                    .if_not_exists()
                    .col(pk_auto(GymProfile::Id))
                    .col(integer(GymProfile::ProfileId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_gym_profile_profile") // Name of the foreign key constraint
                            .from(GymProfile::Table, GymProfile::ProfileId) // From the program table, profile_id column
                            .to(profile::Profile::Table, profile::Profile::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(GymProfile::GymId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_gym_profile_gym")
                            .from(GymProfile::Table, GymProfile::GymId)
                            .to(Gym::Table, Gym::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .add_timestamps()
                    .index(
                        Index::create()
                            .name("idx_gym_profile_unique")
                            .table(GymProfile::Table)
                            .col(GymProfile::ProfileId)
                            .col(GymProfile::GymId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;
        self.create_timestamp_trigger(manager, GymProfile::Table.to_string())
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        self.drop_timestamp_trigger(manager, Gym::Table.to_string())
            .await?;
        self.drop_timestamp_trigger(manager, GymProfile::Table.to_string())
            .await?;
        manager
            .drop_table(Table::drop().table(GymProfile::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Gym::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Gym {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum GymProfile {
    Table,
    Id,
    ProfileId,
    GymId,
}
