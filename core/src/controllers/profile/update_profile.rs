use anyhow::Result;
use fgdb::{
    data::{
        profile::{ProfileData, ProfileUpdateData},
        DbValidationErrors, ResponseData,
    },
    entity::profile::{self, ActiveModel},
};
use fgutils::{constants::VALIDATION_REQUEST_FIELD, verrors};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use tracing::warn;
use validator::{Validate, ValidationErrors};

// should posts also accept params?
// should this create
pub async fn update(
    data: ProfileUpdateData,
    dbc: &DatabaseConnection,
) -> Result<ResponseData<ProfileData>, ValidationErrors> {
    // structural validation
    data.validate()?;

    let profile = profile::Entity::find_by_id(data.id)
        .one(dbc)
        .await
        .map_err(DbValidationErrors::from)?;

    match profile {
        Some(profile) => {
            let mut profile: profile::ActiveModel = profile.into();

            if let Some(name) = data.name {
                profile.name = Set(name);
            }
            if let Some(is_default) = data.is_default {
                profile.is_default = Set(is_default);
            }
            let profile: profile::Model = profile
                .update(dbc)
                .await
                .map_err(DbValidationErrors::from)?;
            return Ok(ResponseData::from_data(profile.into()));
        }
        None => {
            let id = data.id;
            return Err(verrors(
                "id",
                "exists",
                format!("No user with name exists: {id}"),
            ));
        }
    }
}

// Tests done via command
