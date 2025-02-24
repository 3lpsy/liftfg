use anyhow::Result;
use fgdb::{
    data::{
        profile::{ProfileGetParams, ProfileData},
        DbValidationErrors,
    },
    entity::profile::{self},
};
use fgutils::verrors;
use sea_orm::{DatabaseConnection, EntityTrait};
use validator::ValidationErrors;

// gets only accep
pub async fn get(
    params: ProfileGetParams,
    dbc: &DatabaseConnection,
) -> Result<ProfileData, ValidationErrors> {
    if let Some(name) = params.name {
        match profile::Entity::by_name(dbc, &name)
            .await
            .map_err(DbValidationErrors::from)?
        {
            Some(existing) => return Ok(existing.into()),
            None => {
                return Err(verrors(
                    "name",
                    "exists",
                    format!("No user with name exists: {name}"),
                ));
            }
        };
    } else if let Some(id) = params.id {
        match profile::Entity::find_by_id(id)
            .one(dbc)
            .await
            .map_err(DbValidationErrors::from)?
        {
            Some(existing) => return Ok(existing.into()),
            None => {
                return Err(verrors(
                    "id",
                    "exists",
                    format!("No user with name exists: {id}"),
                ));
            }
        };
    } else {
        match profile::Entity::by_default(dbc)
            .await
            .map_err(DbValidationErrors::from)?
        {
            Some(existing) => return Ok(existing.into()),
            None => {
                return Err(verrors(
                    "is_default",
                    "exists",
                    format!("No default user exists: false"),
                ));
            }
        };
    }
}

// Tests done via command
