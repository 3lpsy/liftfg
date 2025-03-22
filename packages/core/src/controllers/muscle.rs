use std::str::FromStr;

use fgdb::{
    data::{
        muscle::{MuscleData, MuscleIndexParams},
        DbValidationErrors, Paginator, ResponseData,
    },
    entity::muscle,
};
use fgutils::{constants::VALIDATION_GENERAL_VALIDATION_CODE, verrors};
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder};
use validator::{Validate, ValidationErrors};

pub async fn index(
    params: MuscleIndexParams,
    dbc: &DatabaseConnection,
) -> Result<ResponseData<Vec<MuscleData>>, ValidationErrors> {
    params.validate()?;

    let pagination = params.pagination.unwrap_or_default();
    let order = params.order.unwrap_or_default();
    let mut query = muscle::Entity::find();
    let order_by = muscle::Column::from_str(&order.order_by).map_err(|_| {
        verrors(
            "order_by",
            VALIDATION_GENERAL_VALIDATION_CODE,
            "Order By field is invalid".to_string(),
        )
    })?;
    query = query.order_by(order_by, order.direction.clone().into());
    let pager = query.paginate(dbc, pagination.size as u64);
    let paginator =
        Paginator::from_db_paginator(&pager, pagination.page, pagination.size, order.direction)
            .await?;
    let rows = pager
        .fetch_page(paginator.page as u64)
        .await
        .map_err(DbValidationErrors::from)?;
    let items: Vec<MuscleData> = rows.clone().into_iter().map(Into::into).collect();
    Ok(ResponseData::from_paginator(items, paginator))
}
