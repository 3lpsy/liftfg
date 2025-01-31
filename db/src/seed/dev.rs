use anyhow::Result;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TransactionTrait};

use crate::entity::user;

pub async fn seed(dbc: DatabaseConnection) -> Result<()> {
    let txn = dbc.begin().await?;
    let name = "TestUser";
    let email = "test@localhost";
    let _user = match user::Entity::by_name(&txn, name).await? {
        Some(existing) => existing,
        None => {
            let user_am = user::ActiveModel {
                name: Set(name.parse()?),
                email: Set(email.parse()?),
                ..Default::default()
            };
            user_am.insert(&txn).await?
        }
    };
    txn.commit().await?;
    Ok(())
}
