use fgdb::{
    data::{
        workout::WorkoutData, DbValidationErrors, DefaultPaginationParams, Paginator, ResponseData,
    },
    entity::workout,
};
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder};
use validator::{Validate, ValidationErrors};

pub async fn index(
    params: DefaultPaginationParams,
    dbc: &DatabaseConnection,
) -> Result<ResponseData<Vec<WorkoutData>>, ValidationErrors> {
    params.validate()?;

    let pagination = params.pagination.unwrap_or_default();

    let pager = workout::Entity::find()
        .order_by(workout::Column::Id, pagination.order.into())
        .paginate(dbc, pagination.size as u64);
    let pagination =
        Paginator::from_db_paginator(&pager, pagination.page, pagination.size, pagination.order)
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
