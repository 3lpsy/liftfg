use anyhow::Result;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};

use crate::entity::user;

pub async fn seed(dbc: DatabaseConnection) -> Result<()> {
    let txn = dbc.begin().await?;
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

    Ok(())
}
