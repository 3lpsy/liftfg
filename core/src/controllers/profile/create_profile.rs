use anyhow::Result;
use fgdb::{
    data::DbValidationErrors,
    entity::profile::{ActiveModel, ProfileCreateData, ProfileResponseData},
};
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use tracing::warn;
use validator::{Validate, ValidationErrors};

// should posts also accept params?
pub async fn create(
    data: ProfileCreateData,
    dbc: &DatabaseConnection,
) -> Result<ProfileResponseData, ValidationErrors> {
    // structural validation
    data.validate()?;
    // logical validation (guard)
    // no two defaults
    // unique name clash
    // for these two simple ones is there any reason to not just let the db handle it?
    // handle
    match ActiveModel::from(data).insert(dbc).await {
        Ok(u) => Ok(u.into()),
        Err(dbe) => {
            let errors: ValidationErrors = DbValidationErrors::from(dbe).into();
            // generic and unhandled, log so we know
            if errors.errors().contains_key("request") {
                warn!("{:?}", &errors);
            }
            Err(errors)
        }
    }
}

// Tests done via command
