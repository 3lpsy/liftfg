use super::common::TableWithTimestamps;
use super::m20220101_000001_create_user as user;

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
                    .col(string(Gym::Name))
                    .add_timestamps()
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(GymUser::Table)
                    .if_not_exists()
                    .col(pk_auto(GymUser::Id))
                    .col(integer(GymUser::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_gym_user_user") // Name of the foreign key constraint
                            .from(GymUser::Table, GymUser::UserId) // From the program table, user_id column
                            .to(user::User::Table, user::User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(GymUser::GymId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_gym_user_gym")
                            .from(GymUser::Table, GymUser::GymId)
                            .to(Gym::Table, Gym::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .add_timestamps()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GymUser::Table).to_owned())
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
enum GymUser {
    Table,
    Id,
    UserId,
    GymId,
}
