use anyhow::Result;
use fgdb::{
    data::{
        profile::{ProfileData, ProfileStoreData},
        DbValidationErrors, ResponseData,
    },
    entity::profile::{self, ActiveModel},
};
use fgutils::constants::VALIDATION_REQUEST_FIELD;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use sea_orm::{DbErr, TransactionTrait};
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
    // <Fn, A, B> -> Result<A, B>
    let inserted = dbc
        .transaction::<_, _, DbErr>(|txn| {
            Box::pin(async move {
                if let Some(is_default) = data.is_default {
                    if is_default {
                        if let Some(existing) = profile::Entity::by_default(txn).await? {
                            let mut am: ActiveModel = existing.into();
                            am.is_default = Set(false);
                            am.update(txn).await?;
                        }
                    }
                }
                let row = ActiveModel::from(data).insert(txn).await?;
                Ok(row)
            })
        })
        .await;

    match inserted {
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
