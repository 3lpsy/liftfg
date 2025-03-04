use anyhow::Result;
use fgdb::{
    data::{
        profile::{ProfileData, ProfileDeleteParams},
        DbValidationErrors, ResponseData,
    },
    entity::profile::{self},
};
use fgutils::verrors;
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};
use validator::{Validate, ValidationErrors};

// gets only accep
pub async fn delete(
    params: ProfileDeleteParams,
    dbc: &DatabaseConnection,
) -> Result<ResponseData<ProfileData>, ValidationErrors> {
    params.validate()?;
    match profile::Entity::find_by_id(params.id)
        .one(dbc)
        .await
        .map_err(DbValidationErrors::from)?
    {
        Some(existing) => {
            let data: ProfileData = existing.clone().into();
            existing
                .delete(dbc)
                .await
                .map_err(DbValidationErrors::from)?;
            return Ok(ResponseData::from_data(data));
        }
        None => {
            let id = params.id;
            return Err(verrors(
                "id",
                "exists",
                format!("No profile with name exists: {id}"),
            ));
        }
    };
}
