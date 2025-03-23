use std::str::FromStr;

use anyhow::Result;
use fgdb::{
    data::{
        profile::ProfileData,
        profile_workout::{
            ProfileWorkoutData, ProfileWorkoutDeleteData, ProfileWorkoutInclude,
            ProfileWorkoutIndexParams, ProfileWorkoutStoreData,
        },
        workout::WorkoutData,
        DbValidationErrors, Paginator, ResponseData,
    },
    entity::{profile, profile_workout, workout},
};
use fgutils::{
    constants::{
        VALIDATION_DATABASE_FIELD, VALIDATION_EXISTS_CODE, VALIDATION_GENERAL_VALIDATION_CODE,
        VALIDATION_PANIC_CODE, VALIDATION_REQUEST_FIELD,
    },
    verrors,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, LoaderTrait, PaginatorTrait, QueryFilter,
    QueryOrder,
};
use sea_orm::{DatabaseConnection, EntityTrait};

use fgdb::entity::common::EntityHelpers;
use tracing::warn;
use validator::{Validate, ValidationErrors};

pub async fn index(
    params: ProfileWorkoutIndexParams,
    dbc: &DatabaseConnection,
) -> Result<ResponseData<Vec<ProfileWorkoutData>>, ValidationErrors> {
    params.validate()?;
    let pagination = params.pagination.unwrap_or_default();
    let order = params.order.unwrap_or_default();
    let includes = params.includes.unwrap_or_default();
    let mut query = profile_workout::Entity::find();

    if let Some(profile_id) = params.profile_id {
        query = query.filter(profile_workout::Column::ProfileId.eq(profile_id));
    }
    if let Some(workout_id) = params.workout_id {
        query = query.filter(profile_workout::Column::WorkoutId.eq(workout_id));
    }
    let order_by: profile_workout::Column = profile_workout::Column::from_str(&order.order_by)
        .map_err(|_| {
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

    let mut items: Vec<ProfileWorkoutData> = rows.clone().into_iter().map(Into::into).collect();

    for include in includes {
        match include {
            ProfileWorkoutInclude::Profile => {
                for (child, item) in rows
                    .load_one(profile::Entity, dbc)
                    .await
                    .map_err(DbValidationErrors::from)?
                    .into_iter()
                    .zip(items.iter_mut())
                {
                    match child {
                        Some(child) => {
                            item.profile = Some(ProfileData::from(child));
                        }
                        None => {
                            return Err(verrors(
                                VALIDATION_DATABASE_FIELD,
                                VALIDATION_PANIC_CODE,
                                "No profile found for profile workout. Unrecoverable state."
                                    .to_string(),
                            ))
                        }
                    }
                }
            }
            ProfileWorkoutInclude::Workout => {
                for (child, item) in rows
                    .load_one(workout::Entity, dbc)
                    .await
                    .map_err(DbValidationErrors::from)?
                    .into_iter()
                    .zip(items.iter_mut())
                {
                    match child {
                        Some(child) => {
                            item.workout = Some(WorkoutData::from(child));
                        }
                        None => {
                            return Err(verrors(
                                VALIDATION_DATABASE_FIELD,
                                VALIDATION_PANIC_CODE,
                                "No workout found for profile workout. Unrecoverable state."
                                    .to_string(),
                            ))
                        }
                    }
                }
            }
        }
    }

    Ok(ResponseData::from_paginator(items, paginator))
}

pub async fn store(
    data: ProfileWorkoutStoreData,
    dbc: &DatabaseConnection,
) -> Result<ResponseData<ProfileWorkoutData>, ValidationErrors> {
    data.validate()?;
    if !profile::Entity::exists(dbc, data.profile_id).await? {
        return Err(verrors(
            "profile_id",
            VALIDATION_EXISTS_CODE,
            format!("No profile with id exists: {}", data.profile_id),
        ));
    }
    if !workout::Entity::exists(dbc, data.workout_id).await? {
        return Err(verrors(
            "workout_id",
            VALIDATION_EXISTS_CODE,
            format!("No workout with id exists: {}", data.workout_id),
        ));
    }

    let inserted = profile_workout::ActiveModel {
        workout_id: ActiveValue::Set(data.workout_id),
        profile_id: ActiveValue::Set(data.profile_id),
        ..Default::default()
    }
    .insert(dbc)
    .await
    .map_err(DbValidationErrors::from);

    match inserted {
        Ok(d) => Ok(ResponseData::from_data(d.into())),
        Err(dbe) => {
            let errors: ValidationErrors = DbValidationErrors::from(dbe).into();
            if errors.errors().contains_key(VALIDATION_REQUEST_FIELD) {
                warn!("{:?}", &errors);
            }
            Err(errors)
        }
    }
}

pub async fn delete(
    data: ProfileWorkoutDeleteData,
    dbc: &DatabaseConnection,
) -> Result<ResponseData<ProfileWorkoutData>, ValidationErrors> {
    data.validate()?;
    if let Some(id) = data.id {
        let row = profile_workout::Entity::by_id(dbc, id).await?;
        if row.is_none() {
            return Err(verrors(
                "id",
                VALIDATION_EXISTS_CODE,
                format!("No profile_workout with id exists: {}", id),
            ));
        }
        let item: ProfileWorkoutData = row.unwrap().into();
        profile_workout::Entity::delete_by_id(item.id)
            .exec(dbc)
            .await
            .map_err(DbValidationErrors::from)?;
        Ok(ResponseData::from_data(item))
    } else if let (Some(profile_id), Some(workout_id)) = (data.profile_id, data.workout_id) {
        let row = profile_workout::Entity::find()
            .filter(profile_workout::Column::ProfileId.eq(profile_id))
            .filter(profile_workout::Column::WorkoutId.eq(workout_id))
            .one(dbc)
            .await
            .map_err(DbValidationErrors::from)?;
        let item: ProfileWorkoutData = row.unwrap().into();
        profile_workout::Entity::delete_by_id(item.id)
            .exec(dbc)
            .await
            .map_err(DbValidationErrors::from)?;
        Ok(ResponseData::from_data(item))
    } else {
        // not theoretically possible w/ validate custom func
        return Err(verrors(
            VALIDATION_REQUEST_FIELD,
            VALIDATION_GENERAL_VALIDATION_CODE,
            format!("Could not handle data provided"),
        ));
    }
}

#[cfg(test)]
mod tests {

    use fgdb::data::{HasIncludes, HasPagination};
    use sea_orm::{ActiveModelTrait, ActiveValue};

    use crate::utils::testutils::setup_test_db_full;

    use super::*;
    #[tokio::test(flavor = "multi_thread")]
    async fn it_invokes_profile_workout_index_with_includes() {
        let (dbc, _test_id) = setup_test_db_full().await.unwrap();
        let workouts = workout::Entity::find().all(&dbc).await.unwrap();
        let profiles = fgdb::seed::dev(&dbc).await.unwrap();
        let _pw1 = profile_workout::ActiveModel {
            workout_id: ActiveValue::Set(workouts[0].id),
            profile_id: ActiveValue::Set(profiles[0].id),
            ..Default::default()
        }
        .insert(&dbc)
        .await
        .unwrap();
        let _pw2 = profile_workout::ActiveModel {
            workout_id: ActiveValue::Set(workouts[1].id),
            profile_id: ActiveValue::Set(profiles[0].id),
            ..Default::default()
        }
        .insert(&dbc)
        .await
        .unwrap();
        let _pw3 = profile_workout::ActiveModel {
            workout_id: ActiveValue::Set(workouts[0].id),
            profile_id: ActiveValue::Set(profiles[1].id),
            ..Default::default()
        }
        .insert(&dbc)
        .await
        .unwrap();
        let req = ProfileWorkoutIndexParams::default()
            .with_page(0)
            .with_include(ProfileWorkoutInclude::Profile)
            .with_include(ProfileWorkoutInclude::Workout);
        let res = index(req, &dbc).await.unwrap();
        assert!(res.data.as_ref().unwrap().len() == 3);
        res.data.as_ref().unwrap().iter().for_each(|i| {
            assert!(i.profile.is_some());
            assert!(i.workout.is_some());
        });
    }
}
