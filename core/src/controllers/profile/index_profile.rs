use anyhow::Result;
use fgdb::{
    data::{
        profile::ProfileData, DbValidationErrors, DefaultPaginationParams, Paginator, ResponseData,
    },
    entity::profile::{self},
};
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder};
use validator::{Validate, ValidationErrors};

// gets only accep
pub async fn index(
    params: DefaultPaginationParams,
    dbc: &DatabaseConnection,
) -> Result<ResponseData<Vec<ProfileData>>, ValidationErrors> {
    params.validate()?;

    let pagination = params.pagination.unwrap_or_default();

    // TODOcasting i32 to u64
    let pager = profile::Entity::find()
        .order_by(profile::Column::Id, pagination.order.into())
        .paginate(dbc, pagination.size as u64);
    let pagination =
        Paginator::from_db_paginator(&pager, pagination.page, pagination.size, pagination.order)
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
