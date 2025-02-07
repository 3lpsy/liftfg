use anyhow::Result;
use fgdb::entity::{
    profile::{ActiveModel, ProfileCreateData, ProfileResponseData},
    wrappers::{DbValidationErrors, ResponseData},
};
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use tracing::warn;
use validator::Validate;

pub async fn create_profile(
    data: ProfileCreateData,
    dbc: &DatabaseConnection,
) -> Result<ResponseData<ProfileResponseData>> {
    // structural validation
    if let Err(e) = data.validate() {
        return Ok(ResponseData::new(None, Some(e)));
    }
    // logical validation: uniqueness, permission

    // handle
    let response = match Into::<ActiveModel>::into(data).insert(dbc).await {
        Ok(u) => ResponseData::new(Some(u.into()), None),
        Err(dbe) => {
            warn!("{:?}", &dbe);
            ResponseData::new(None, Some(DbValidationErrors::from(dbe).into()))
        }
    };
    // response
    Ok(response)
}
