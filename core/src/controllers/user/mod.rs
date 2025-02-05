use anyhow::Result;
use fgdb::entity::{
    user::{ActiveModel, UserCreateData, UserResponseData},
    wrappers::{DbValidationErrors, ResponseData},
};
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use tracing::warn;
use validator::Validate;

pub async fn create_user(
    data: UserCreateData,
    dbc: &DatabaseConnection,
) -> Result<ResponseData<UserResponseData>> {
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
