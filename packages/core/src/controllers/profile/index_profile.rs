use anyhow::Result;
use fgdb::{
    data::{profile::ProfileData, DbValidationErrors, DefaultParams, Paginator, ResponseData},
    entity::profile::{self},
};
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder};
use validator::{Validate, ValidationErrors};

pub async fn index(
    params: DefaultParams,
    dbc: &DatabaseConnection,
) -> Result<ResponseData<Vec<ProfileData>>, ValidationErrors> {
    params.validate()?;
    let pagination = params.pagination.unwrap_or_default();
    let order = params.order.unwrap_or_default();

    // TODOcasting i32 to u64
    let pager = profile::Entity::find()
        .order_by(profile::Column::Id, order.direction.clone().into())
        .paginate(dbc, pagination.size as u64);

    let pagination =
        Paginator::from_db_paginator(&pager, pagination.page, pagination.size, order.direction)
            .await?;
    let profiles = pager
        .fetch_page(pagination.page as u64)
        .await
        .map_err(DbValidationErrors::from)?;

    Ok(ResponseData::from_paginator(
        profiles.into_iter().map(Into::into).collect(),
        pagination,
    ))
}

// test for validation
