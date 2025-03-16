use std::collections::HashMap;

use fgdb::{
    data::{
        muscle::MuscleData,
        profile::ProfileData,
        profile_workout::ProfileWorkoutData,
        workout::{WorkoutData, WorkoutInclude, WorkoutIndexParams},
        workout_muscle::{WorkoutMuscleData, WorkoutMuscleInclude},
        DbValidationErrors, Paginator, ResponseData,
    },
    entity::{muscle, profile, profile_workout, workout, workout_muscle},
};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, PaginatorTrait, QueryFilter,
    QueryOrder,
};
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

    // pagination and relationships aren't really compat
    // so let's paginate the root model and use LoaderTrait for direct relationships
    // and wall hacks for nested relationships
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
            WorkoutInclude::WorkoutMuscle(nested_includes) => {
                let child_rows = rows
                    .load_many(workout_muscle::Entity, dbc)
                    .await
                    .map_err(DbValidationErrors::from)?;

                // composite child itmes vec of vec for each item
                let mut child_items_comp: Vec<Vec<WorkoutMuscleData>> = child_rows
                    .into_iter()
                    .map(|x| x.into_iter().map(WorkoutMuscleData::from).collect())
                    .collect();

                if let Some(nested_includes) = nested_includes {
                    for nested_include in nested_includes {
                        match nested_include {
                            WorkoutMuscleInclude::Muscle => {
                                let nested_ids: Vec<i32> = child_items_comp
                                    .clone()
                                    .into_iter()
                                    .flat_map(|inner| inner.into_iter().map(|item| item.muscle_id))
                                    .collect();

                                let nested_rows = muscle::Entity::find()
                                    .filter(muscle::Column::Id.is_in(nested_ids))
                                    .all(dbc)
                                    .await
                                    .map_err(DbValidationErrors::from)?;

                                let nested_map: HashMap<i32, MuscleData> = nested_rows
                                    .into_iter()
                                    .map(|r| (r.id, MuscleData::from(r)))
                                    .collect();

                                child_items_comp.iter_mut().for_each(|child_comp_items| {
                                    child_comp_items.iter_mut().for_each(|child_item| {
                                        child_item.muscle =
                                            nested_map.get(&child_item.muscle_id).cloned();
                                    });
                                });
                            }
                            _ => unimplemented!(),
                        }
                    }
                }

                child_items_comp
                    .into_iter()
                    .zip(items.iter_mut())
                    .for_each(|(child_items, item)| item.workout_muscle = Some(child_items));
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

// TODO tests
