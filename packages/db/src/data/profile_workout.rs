#[cfg(feature = "db")]
use crate::entity::profile_workout as entity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{ResponsableData, ResponseData};

// Responses
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProfileWorkoutData {
    pub id: i32,
    pub workout_id: i32,
    pub profile_id: i32,
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
            profile_id: model.profile_id,
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
