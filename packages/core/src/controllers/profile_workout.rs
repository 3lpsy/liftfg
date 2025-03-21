use anyhow::Result;
use fgdb::{
    data::{
        profile_workout::{ProfileWorkoutData, ProfileWorkoutDeleteData, ProfileWorkoutStoreData},
        DbValidationErrors, ResponseData,
    },
    entity::{profile, profile_workout, workout},
};
use fgutils::{
    constants::{VALIDATION_GENERAL_VALIDATION_CODE, VALIDATION_REQUEST_FIELD},
    verrors,
};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, QueryFilter};
use sea_orm::{DatabaseConnection, EntityTrait};

use fgdb::entity::common::EntityHelpers;
use tracing::warn;
use validator::{Validate, ValidationErrors};

// should posts also accept params?
// should this create
pub async fn store(
    data: ProfileWorkoutStoreData,
    dbc: &DatabaseConnection,
) -> Result<ResponseData<ProfileWorkoutData>, ValidationErrors> {
    data.validate()?;
    if !profile::Entity::exists(dbc, data.profile_id).await? {
        return Err(verrors(
            "profile_id",
            "exists",
            format!("No profile with id exists: {}", data.profile_id),
        ));
    }
    if !workout::Entity::exists(dbc, data.workout_id).await? {
        return Err(verrors(
            "workout_id",
            "exists",
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
                "exists",
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
