use fgdb::{
    data::{workout::WorkoutData, DbValidationErrors, DefaultParams, Paginator, ResponseData},
    entity::workout,
};
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder};
use validator::{Validate, ValidationErrors};

pub async fn index(
    params: DefaultParams,
    dbc: &DatabaseConnection,
) -> Result<ResponseData<Vec<WorkoutData>>, ValidationErrors> {
    params.validate()?;

    let pagination = params.pagination.unwrap_or_default();
    let order = params.order.unwrap_or_default();

    let pager = workout::Entity::find()
        .order_by(workout::Column::Id, order.direction.into())
        .paginate(dbc, pagination.size as u64);
    let pagination =
        Paginator::from_db_paginator(&pager, pagination.page, pagination.size, order.direction)
            .await?;
    let items = pager
        .fetch_page(pagination.page as u64)
        .await
        .map_err(DbValidationErrors::from)?;

    Ok(ResponseData::from_paginator(
        items.into_iter().map(Into::into).collect(),
        pagination,
    ))
}
