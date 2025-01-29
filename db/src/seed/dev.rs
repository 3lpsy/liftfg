use anyhow::Result;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};

use crate::entity::user;

pub async fn seed(dbc: DatabaseConnection) -> Result<()> {
    let txn = dbc.begin().await?;
    // When `id` column have conflicting value, do nothing
    let on_conflict = OnConflict::column(Column::Id).do_nothing().to_owned();
    let name = "TestUser";
    let user = match user::Entity::find()
        .filter(user::Column::Name.eq(name))
        .one(&txn)
        .await?
    {
        Some(existing) => existing,
        None => {
            let user_am = user::ActiveModel {
                name: Set(name.parse()?),
                ..Default::default()
            };
            user_am.insert(&txn).await?
        }
    };
    // setup many to many

    Ok(())
}
