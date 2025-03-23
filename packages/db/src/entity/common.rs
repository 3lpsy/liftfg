use fgutils::{constants::VALIDATION_EXISTS_CODE, verrors};
use sea_orm::{DatabaseConnection, EntityTrait, PrimaryKeyTrait};
use sea_orm_migration::async_trait::async_trait;
use validator::ValidationErrors;

use crate::data::DbValidationErrors;

#[async_trait]
pub trait EntityHelpers<E>
where
    E: EntityTrait,
{
    async fn exists(dbc: &DatabaseConnection, id: i32) -> Result<bool, ValidationErrors>;
    async fn exists_or_err(dbc: &DatabaseConnection, id: i32) -> Result<bool, ValidationErrors>;

    async fn by_id(dbc: &DatabaseConnection, id: i32)
        -> Result<Option<E::Model>, ValidationErrors>;
    async fn by_id_or_err(dbc: &DatabaseConnection, id: i32) -> Result<E::Model, ValidationErrors>;
}

#[async_trait]
impl<E> EntityHelpers<E> for E
where
    E: EntityTrait,
    E::PrimaryKey: PrimaryKeyTrait<ValueType = i32>,
{
    async fn exists(dbc: &DatabaseConnection, id: i32) -> Result<bool, ValidationErrors> {
        Ok(E::find_by_id(id)
            .one(dbc)
            .await
            .map_err(DbValidationErrors::from)?
            .is_some())
    }
    async fn exists_or_err(dbc: &DatabaseConnection, id: i32) -> Result<bool, ValidationErrors> {
        match E::exists(dbc, id).await? {
            true => Ok(true),
            false => Err(verrors(
                "id",
                VALIDATION_EXISTS_CODE,
                format!("Entity with ID {} does not exist", id),
            )),
        }
    }
    async fn by_id(
        dbc: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<E::Model>, ValidationErrors> {
        Ok(E::find_by_id(id)
            .one(dbc)
            .await
            .map_err(DbValidationErrors::from)?)
    }

    async fn by_id_or_err(dbc: &DatabaseConnection, id: i32) -> Result<E::Model, ValidationErrors> {
        match E::by_id(dbc, id).await? {
            Some(e) => Ok(e),
            None => Err(verrors(
                "id",
                VALIDATION_EXISTS_CODE,
                format!("Entity with ID {} does not exist", id),
            )),
        }
    }
}
