use std::{collections::HashMap, str::FromStr};

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
use fgutils::{constants::VALIDATION_GENERAL_VALIDATION_CODE, verrors};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, JoinType, LoaderTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, RelationTrait,
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

    let mut query = workout::Entity::find();
    if let Some(profile_id) = params.profile_id {
        // filter on relationship workout_profile where workout_profile.profile_id is profile_id
        // can't use relationship
        query = query
            .join(JoinType::InnerJoin, workout::Relation::ProfileWorkout.def())
            .filter(profile_workout::Column::ProfileId.eq(profile_id));
    }

    let order_by = workout::Column::from_str(&order.order_by).map_err(|_| {
        verrors(
            "order_by",
            VALIDATION_GENERAL_VALIDATION_CODE,
            "Order By field is invalid".to_string(),
        )
    })?;
    query = query.order_by(order_by, order.direction.clone().into());
    let pager = query.paginate(dbc, pagination.size as u64);

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
                            // in reality, you can't query a nested workout on a muscle so
                            // this should val error
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
                        item.profiles = Some(
                            child_rows
                                .into_iter()
                                .map(|i| ProfileData::from(i))
                                .collect(),
                        );
                    });
            }
        }
    }

    Ok(ResponseData::from_paginator(items, paginator))
}

#[cfg(test)]
mod tests {

    use fgdb::data::{HasIncludes, HasPagination};
    use sea_orm::{ActiveModelTrait, ActiveValue};

    use crate::utils::testutils::setup_test_db_full;

    use super::*;

    #[tokio::test(flavor = "multi_thread")] // not really faster
    async fn it_invokes_workout_index_first_page() {
        let (dbc, _test_id) = setup_test_db_full().await.unwrap();
        let all = workout::Entity::find().all(&dbc).await.unwrap();
        assert!(all.len() > 1);
        // default size 10
        // first page is 0
        let req = WorkoutIndexParams::default().with_page(0);
        let page_size = req.pagination.as_ref().unwrap().size;
        let res = index(req, &dbc).await.unwrap();
        assert!(res.data.as_ref().unwrap().len() as i32 <= page_size);
        assert_eq!(all.len(), res.data.as_ref().unwrap().len());
        res.data.as_ref().unwrap().iter().for_each(|i| {
            assert!(i.profiles.is_none());
            assert!(i.profile_workout.is_none());
            assert!(i.workout_muscle.is_none());
        });
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn it_invokes_workout_index_with_includes() {
        let (dbc, _test_id) = setup_test_db_full().await.unwrap();
        let all = workout::Entity::find().all(&dbc).await.unwrap();
        assert!(all.len() > 1);
        let req = WorkoutIndexParams::default()
            .with_page(0)
            .with_include(WorkoutInclude::ProfileWorkout)
            .with_include(WorkoutInclude::Profile);

        let page_size = req.pagination.as_ref().unwrap().size;
        let res = index(req, &dbc).await.unwrap();
        assert!(res.data.as_ref().unwrap().len() as i32 <= page_size);
        assert_eq!(all.len(), res.data.as_ref().unwrap().len());
        res.data.as_ref().unwrap().iter().for_each(|i| {
            assert!(i.profiles.is_some());
            // no assigned profiles for default setup
            assert!(i.profiles.as_ref().unwrap().is_empty());
            assert!(i.profile_workout.is_some());
            // same as above (empty) but is the pivot table
            assert!(i.profile_workout.as_ref().unwrap().is_empty());
            assert!(i.workout_muscle.is_none());
        });
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn it_invokes_workout_index_with_workout_muscle() {
        let (dbc, _test_id) = setup_test_db_full().await.unwrap();
        let all = workout::Entity::find().all(&dbc).await.unwrap();
        assert!(all.len() > 1);
        let req = WorkoutIndexParams::default()
            .with_page(0)
            .with_include(WorkoutInclude::WorkoutMuscle(None));
        let page_size = req.pagination.as_ref().unwrap().size;
        let res = index(req, &dbc).await.unwrap();
        assert!(res.data.as_ref().unwrap().len() as i32 <= page_size);
        assert_eq!(all.len(), res.data.as_ref().unwrap().len());
        res.data.as_ref().unwrap().iter().for_each(|i| {
            // by default, there's data
            assert!(i.workout_muscle.is_some());
            assert!(i.workout_muscle.as_ref().unwrap().len() > 0);
            // assert no nest
            assert!(i.workout_muscle.as_ref().unwrap()[0].muscle.is_none());
        });
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn it_invokes_workout_index_with_workout_muscle_nested_muscle() {
        let (dbc, _test_id) = setup_test_db_full().await.unwrap();
        let all = workout::Entity::find().all(&dbc).await.unwrap();
        assert!(all.len() > 1);
        let req =
            WorkoutIndexParams::default()
                .with_page(0)
                .with_include(WorkoutInclude::WorkoutMuscle(Some(vec![
                    WorkoutMuscleInclude::Muscle,
                ])));
        let page_size = req.pagination.as_ref().unwrap().size;
        let res = index(req, &dbc).await.unwrap();
        assert!(res.data.as_ref().unwrap().len() as i32 <= page_size);
        assert_eq!(all.len(), res.data.as_ref().unwrap().len());
        res.data.as_ref().unwrap().iter().for_each(|ii| {
            assert!(ii.workout_muscle.is_some());
            assert!(ii.workout_muscle.as_ref().unwrap().len() > 0);
            ii.workout_muscle.as_ref().unwrap().iter().for_each(|jj| {
                assert!(jj.muscle.is_some());
                assert!(jj.muscle.as_ref().unwrap().id > 0);
            });
        });
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn it_invokes_workout_index_for_profile() {
        let (dbc, _test_id) = setup_test_db_full().await.unwrap();
        let all = workout::Entity::find().all(&dbc).await.unwrap();
        let profs = fgdb::seed::dev(&dbc).await.unwrap();
        profile_workout::ActiveModel {
            workout_id: ActiveValue::Set(all[0].id),
            profile_id: ActiveValue::Set(profs[0].id),
            ..Default::default()
        }
        .insert(&dbc)
        .await
        .unwrap();
        profile_workout::ActiveModel {
            workout_id: ActiveValue::Set(all[1].id),
            profile_id: ActiveValue::Set(profs[0].id),
            ..Default::default()
        }
        .insert(&dbc)
        .await
        .unwrap();

        let mut req = WorkoutIndexParams::default()
            .with_page(0)
            .with_include(WorkoutInclude::ProfileWorkout);
        req.profile_id = Some(profs[0].id);
        let res = index(req, &dbc).await.unwrap();
        assert_eq!(2, res.data.as_ref().unwrap().len());
        res.data.as_ref().unwrap().iter().for_each(|ii| {
            assert!(ii.profile_workout.is_some());
            ii.profile_workout.as_ref().unwrap().iter().for_each(|jj| {
                assert!(jj.profile_id == profs[0].id);
            });
        });
    }
}
