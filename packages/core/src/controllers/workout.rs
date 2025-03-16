use fgdb::{
    data::{
        profile::ProfileData,
        profile_workout::ProfileWorkoutData,
        workout::{WorkoutData, WorkoutInclude, WorkoutIndexParams},
        workout_muscle::WorkoutMuscleData,
        DbValidationErrors, Paginator, ResponseData,
    },
    entity::{profile, profile_workout, workout, workout_muscle},
};
use sea_orm::{DatabaseConnection, EntityTrait, LoaderTrait, PaginatorTrait, QueryOrder};
use validator::{Validate, ValidationErrors};

pub async fn index(
    params: WorkoutIndexParams,
    dbc: &DatabaseConnection,
) -> Result<ResponseData<Vec<WorkoutData>>, ValidationErrors> {
    params.validate()?;

    let pagination = params.pagination.unwrap_or_default();
    let order = params.order.unwrap_or_default();
    let includes = params.includes.unwrap_or_default();

    let pager = workout::Entity::find()
        .order_by(workout::Column::Id, order.direction.clone().into())
        .paginate(dbc, pagination.size as u64);

    let paginator =
        Paginator::from_db_paginator(&pager, pagination.page, pagination.size, order.direction)
            .await?;

    let rows = pager
        .fetch_page(paginator.page as u64)
        .await
        .map_err(DbValidationErrors::from)?;

    // load root rows into data type
    let mut items: Vec<WorkoutData> = rows.clone().into_iter().map(Into::into).collect();
    // alternative to find_with_related that uses two queries
    for include in includes {
        match include {
            WorkoutInclude::WorkoutMuscle(child_includes) => {
                rows.load_many(workout_muscle::Entity, dbc)
                    .await
                    .map_err(DbValidationErrors::from)?
                    .into_iter()
                    .zip(items.iter_mut())
                    .for_each(|(child_rows, item)| {
                        item.workout_muscle = Some(
                            child_rows
                                .into_iter()
                                .map(WorkoutMuscleData::from)
                                .collect(),
                        )
                    });
            }
            WorkoutInclude::ProfileWorkout => {
                rows.load_many(profile_workout::Entity, dbc)
                    .await
                    .map_err(DbValidationErrors::from)?
                    .into_iter()
                    .zip(items.iter_mut())
                    .for_each(|(child_rows, item)| {
                        item.profile_workout = Some(
                            child_rows
                                .into_iter()
                                .map(ProfileWorkoutData::from)
                                .collect(),
                        )
                    });
            }
            WorkoutInclude::Profile => {
                rows.load_many_to_many(profile::Entity, profile_workout::Entity, dbc)
                    .await
                    .map_err(DbValidationErrors::from)?
                    .into_iter()
                    .zip(items.iter_mut())
                    .for_each(|(child_rows, item)| {
                        // n-to-n relationship but only really can have 1
                        item.profile = child_rows.into_iter().next().map(ProfileData::from);
                    });
            }
        }
    }

    Ok(ResponseData::from_paginator(items, paginator))
}
