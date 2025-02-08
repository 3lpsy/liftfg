use anyhow::Result;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TransactionTrait};

use crate::entity::profile;

pub async fn seed(dbc: DatabaseConnection) -> Result<()> {
    let txn = dbc.begin().await?;
    let name = "TestProfile";
    let _profile = match profile::Entity::by_name(&txn, name).await? {
        Some(existing) => existing,
        None => {
            let profile_am = profile::ActiveModel {
                name: Set(name.parse()?),
                is_default: Set(true),
                ..Default::default()
            };
            profile_am.insert(&txn).await?
        }
    };
    txn.commit().await?;
    Ok(())
}
