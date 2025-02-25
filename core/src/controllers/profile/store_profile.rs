use anyhow::Result;
use fgdb::{
    data::{
        profile::{ProfileData, ProfileStoreData},
        DbValidationErrors, ResponseData,
    },
    entity::profile::ActiveModel,
};
use fgutils::constants::VALIDATION_REQUEST_FIELD;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use tracing::warn;
use validator::{Validate, ValidationErrors};

// should posts also accept params?
// should this create
pub async fn store(
    data: ProfileStoreData,
    dbc: &DatabaseConnection,
) -> Result<ResponseData<ProfileData>, ValidationErrors> {
    // structural validation
    data.validate()?;

    // application validation
    // main failure scenarios: no two defaults, unique name clash
    // is there any reason to not just let the db handle it?
    match ActiveModel::from(data).insert(dbc).await {
        Ok(d) => Ok(ResponseData::from_data(d.into())),
        Err(dbe) => {
            let errors: ValidationErrors = DbValidationErrors::from(dbe).into();
            // generic and unhandled, log so we know
            if errors.errors().contains_key(VALIDATION_REQUEST_FIELD) {
                warn!("{:?}", &errors);
            }
            Err(errors)
        }
    }
}

// Tests done via command
