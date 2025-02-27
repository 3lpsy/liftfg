use anyhow::Result;
use fgdb::{
    data::{
        profile::{ProfileData, ProfileUpdateData},
        DbValidationErrors, ResponseData,
    },
    entity::profile::{self},
};
use fgutils::{constants::VALIDATION_REQUEST_FIELD, verrors};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set, TransactionTrait};
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

    let existing = profile::Entity::find_by_id(data.id)
        .one(dbc)
        .await
        .map_err(DbValidationErrors::from)?;

    match existing {
        Some(existing) => {
            // set the params
            let mut existing: profile::ActiveModel = existing.into();
            if let Some(name) = data.name {
                existing.name = Set(name);
            }
            if let Some(is_default) = data.is_default {
                existing.is_default = Set(is_default);
            }
            let updated = dbc
                .transaction::<_, _, DbErr>(|txn| {
                    Box::pin(async move {
                        // check for other default if it exists
                        if let Some(is_default) = data.is_default {
                            if is_default {
                                if let Some(other) = profile::Entity::by_default(txn).await? {
                                    let mut am: profile::ActiveModel = other.into();
                                    am.is_default = Set(false);
                                    am.update(txn).await?;
                                }
                            }
                        }
                        let row = existing.update(txn).await?;
                        Ok(row)
                    })
                })
                .await;
            match updated {
                Ok(d) => Ok(ResponseData::from_data(d.into())),
                Err(dbe) => {
                    let errors: ValidationErrors = DbValidationErrors::from(dbe).into();
                    if errors.errors().contains_key(VALIDATION_REQUEST_FIELD) {
                        warn!("{:?}", &errors);
                    }
                    Err(errors)
                }
            }
        }
        None => {
            let id = data.id;
            return Err(verrors(
                "id",
                "exists",
                format!("No profile with name exists: {id}"),
            ));
        }
    }
}

// Tests done via command
