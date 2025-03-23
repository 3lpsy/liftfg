use anyhow::Result;
use fgdb::{
    data::{
        profile::{ProfileData, ProfileShowParams},
        DbValidationErrors, ResponseData,
    },
    entity::profile::{self},
};
use fgutils::{constants::VALIDATION_EXISTS_CODE, verrors};
use sea_orm::{DatabaseConnection, EntityTrait};
use validator::{Validate, ValidationErrors};

// gets only accep
pub async fn show(
    params: ProfileShowParams,
    dbc: &DatabaseConnection,
) -> Result<ResponseData<ProfileData>, ValidationErrors> {
    params.validate()?;
    if let Some(name) = params.name {
        match profile::Entity::by_name(dbc, &name)
            .await
            .map_err(DbValidationErrors::from)?
        {
            Some(existing) => return Ok(ResponseData::from_data(existing.into())),
            None => {
                return Err(verrors(
                    "name",
                    VALIDATION_EXISTS_CODE,
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
            Some(existing) => return Ok(ResponseData::from_data(existing.into())),
            None => {
                return Err(verrors(
                    "id",
                    VALIDATION_EXISTS_CODE,
                    format!("No user with name exists: {id}"),
                ));
            }
        };
    } else {
        match profile::Entity::by_default(dbc)
            .await
            .map_err(DbValidationErrors::from)?
        {
            Some(existing) => return Ok(ResponseData::from_data(existing.into())),
            None => {
                return Err(verrors(
                    "is_default",
                    VALIDATION_EXISTS_CODE,
                    format!("No default user exists: false"),
                ));
            }
        };
    }
}
