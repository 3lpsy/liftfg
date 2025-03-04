#[cfg(feature = "db")]
// only db
use anyhow::Result;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use crate::entity::profile;

pub fn default_program_data() -> Vec<String> {
    let p = vec![
        "Upper Body Push",
        "Upper Body Pull",
        "Upper Body",
        "Lower Body",
        "Full Body Push",
        "Full Body Pull",
        "Full Body",
        "Chest",
        "Back",
        "Shoulders",
        "Arms",
    ];
    p.iter().map(|i| i.to_string()).collect()
}

// seeding should be done in migrations as schema may change
#[cfg(feature = "db")]
pub async fn dev(dbc: DatabaseConnection) -> Result<()> {
    let name = "TestProfile";
    let _profile = match profile::Entity::by_name(&dbc, name).await? {
        Some(existing) => existing,
        None => {
            let profile_am = profile::ActiveModel {
                name: Set(name.parse()?),
                is_default: Set(true),
                ..Default::default()
            };
            profile_am.insert(&dbc).await?
        }
    };
    let name = "TestProfile2";

    let _profile2 = match profile::Entity::by_name(&dbc, name).await? {
        Some(existing) => existing,
        None => {
            let profile_am = profile::ActiveModel {
                name: Set(name.parse()?),
                is_default: Set(false),
                ..Default::default()
            };
            profile_am.insert(&dbc).await?
        }
    };
    Ok(())
}
