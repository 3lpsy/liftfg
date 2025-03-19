#[cfg(feature = "db")]
// only db
use anyhow::Result;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use crate::entity::profile;

// seeding should be done in migrations as schema may change
#[cfg(feature = "db")]
pub async fn dev(dbc: &DatabaseConnection) -> Result<Vec<profile::Model>> {
    let name = "TestProfile";
    let mut profs = vec![];
    let p = match profile::Entity::by_name(dbc, name).await? {
        Some(existing) => existing,
        None => {
            let profile_am = profile::ActiveModel {
                name: Set(name.parse()?),
                is_default: Set(true),
                ..Default::default()
            };
            profile_am.insert(dbc).await?
        }
    };
    profs.push(p);
    let name = "TestProfile2";

    let p = match profile::Entity::by_name(dbc, name).await? {
        Some(existing) => existing,
        None => {
            let profile_am = profile::ActiveModel {
                name: Set(name.parse()?),
                is_default: Set(false),
                ..Default::default()
            };
            profile_am.insert(dbc).await?
        }
    };
    profs.push(p);
    Ok(profs)
}
