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
    async fn by_id(dbc: &DatabaseConnection, id: i32)
        -> Result<Option<E::Model>, ValidationErrors>;
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

    async fn by_id(
        dbc: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<E::Model>, ValidationErrors> {
        Ok(E::find_by_id(id)
            .one(dbc)
            .await
            .map_err(DbValidationErrors::from)?)
    }
}
