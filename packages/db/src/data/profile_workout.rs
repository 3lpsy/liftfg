#[cfg(feature = "db")]
use crate::entity::profile_workout as entity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use super::{
    profile::ProfileData, workout::WorkoutData, HasIncludes, HasOrder, HasPagination, Includable,
    Order, Pagination, ResponsableData, ResponseData,
};

// Requests
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProfileWorkoutInclude {
    Profile,
    Workout,
}
impl Includable for ProfileWorkoutInclude {}

#[derive(Default, Clone, Debug, Validate, Serialize, Deserialize)]
pub struct ProfileWorkoutIndexParams {
    #[validate(range(min = 1, max = 256, message = "Profile ID must be between 1 and 256"))]
    pub profile_id: Option<i32>,
    #[validate(range(min = 1, max = 256, message = "Workout ID must be between 1 and 256"))]
    pub workout_id: Option<i32>,
    #[validate(nested)]
    pub pagination: Option<Pagination>,
    #[validate(nested)]
    pub order: Option<Order>,
    #[validate(length(max = 2, message = "Max length of array is 3."))]
    pub includes: Option<Vec<ProfileWorkoutInclude>>,
}

impl ProfileWorkoutIndexParams {
    pub fn with_profile_id(mut self, id: i32) -> Self {
        self.profile_id = Some(id);
        self
    }
    // fn with_workout_id(mut self, id: i32) -> Self {
    //     self.workout_id = Some(id);
    //     self
    // }
}

#[derive(Debug, Validate, Serialize, Deserialize, Default, Clone)]
pub struct ProfileWorkoutStoreData {
    #[validate(range(min = 1, message = "Workout ID must be greater than 1"))]
    pub workout_id: i32,
    #[validate(range(min = 1, message = "Profile ID must be greater than 1"))]
    pub profile_id: i32,
}

fn validate_delete_data(data: &ProfileWorkoutDeleteData) -> Result<(), ValidationError> {
    let has_id = data.id.is_some();
    let has_workout_profile_ids = data.workout_id.is_some() && data.profile_id.is_some();
    if has_id && has_workout_profile_ids {
        return Err(ValidationError::new(
            "Cannot provide both ID and Workout ID / Profile ID simultaneously",
        ));
    }
    if !has_id && !has_workout_profile_ids {
        return Err(ValidationError::new(
            "Must provide either ID or both Workout ID and Profile ID",
        ));
    }
    Ok(())
}

#[derive(Debug, Validate, Serialize, Deserialize, Default, Clone)]
#[validate(schema(function = "validate_delete_data", skip_on_field_errors = true))]
pub struct ProfileWorkoutDeleteData {
    #[validate(range(min = 1, message = "Workout ID must be greater than 1"))]
    pub workout_id: Option<i32>,
    #[validate(range(min = 1, message = "Profile ID must be greater than 1"))]
    pub profile_id: Option<i32>,
    #[validate(range(min = 1, message = "ProfileWorkout ID must be greater than 1"))]
    pub id: Option<i32>,
}

// Responses
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProfileWorkoutData {
    pub id: i32,
    pub workout_id: i32,
    pub workout: Option<WorkoutData>,
    pub profile_id: i32,
    pub profile: Option<ProfileData>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// impls
#[cfg(feature = "db")]
impl From<entity::Model> for ProfileWorkoutData {
    fn from(model: entity::Model) -> Self {
        Self {
            id: model.id,
            workout_id: model.workout_id,
            workout: None,
            profile_id: model.profile_id,
            profile: None,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<ProfileWorkoutData> for ResponseData<ProfileWorkoutData> {
    fn from(data: ProfileWorkoutData) -> Self {
        ResponseData {
            data: Some(data),
            errors: None,
            paginator: None,
        }
    }
}

impl ResponsableData for ProfileWorkoutData {}
impl ResponsableData for Vec<ProfileWorkoutData> {}

impl HasIncludes<ProfileWorkoutInclude> for ProfileWorkoutIndexParams {
    fn includes(&mut self) -> &mut Option<Vec<ProfileWorkoutInclude>> {
        &mut self.includes
    }
}

impl HasPagination for ProfileWorkoutIndexParams {
    fn pagination(&mut self) -> &mut Option<Pagination> {
        &mut self.pagination
    }
}

impl HasOrder for ProfileWorkoutIndexParams {
    fn order(&mut self) -> &mut Option<Order> {
        &mut self.order
    }
}
