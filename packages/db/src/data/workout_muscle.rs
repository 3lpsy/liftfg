use crate::data::enums::ExercisePromptStrategy;
#[cfg(feature = "db")]
use crate::entity::workout_muscle as entity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{muscle::MuscleData, profile::ProfileData, ResponsableData, ResponseData};
// requests
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WorkoutMuscleInclude {
    Workout,
    Muscle,
}

#[derive(Debug, Default, Validate, Serialize, Deserialize, Clone, PartialEq)]
pub struct WorkoutMuscleStoreData {
    #[validate(range(min = 1, max = 4096, message = "Workout ID must be greater than 1"))]
    pub workout_id: i32,
    #[validate(range(min = 1, max = 4096, message = "Muscle ID must be greater than 1"))]
    pub muscle_id: i32,
    #[validate(range(min = 1, max = 64, message = "Volume must be between 1 and 64"))]
    pub volume: i32,
    #[validate(range(min = 1, max = 10, message = "Priority must be between 1 and 10"))]
    pub priority: i32,
    #[validate(range(
        min = 1,
        max = 64,
        message = "Exercise set split must be between 1 and 64"
    ))]
    pub exercise_set_split: Option<i32>,
    pub exercise_prompt_strategy: Option<ExercisePromptStrategy>,
}

// Responses
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkoutMuscleData {
    pub id: i32,
    pub workout_id: i32,
    pub muscle_id: i32,
    pub volume: i32,
    pub priority: i32,
    pub exercise_set_split: Option<i32>,
    pub exercise_prompt_strategy: Option<ExercisePromptStrategy>,

    // timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // relationships
    pub muscle: Option<MuscleData>,
    pub profile: Option<ProfileData>,
}

// impls
#[cfg(feature = "db")]
impl From<entity::Model> for WorkoutMuscleData {
    fn from(model: entity::Model) -> Self {
        Self {
            id: model.id,
            workout_id: model.workout_id,
            muscle_id: model.muscle_id,
            priority: model.muscle_id,
            exercise_set_split: model.exercise_set_split,
            exercise_prompt_strategy: model.exercise_prompt_strategy,
            volume: model.volume,
            muscle: None,
            profile: None,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<WorkoutMuscleData> for ResponseData<WorkoutMuscleData> {
    fn from(data: WorkoutMuscleData) -> Self {
        ResponseData {
            data: Some(data),
            errors: None,
            paginator: None,
        }
    }
}

impl ResponsableData for WorkoutMuscleData {}
impl ResponsableData for Vec<WorkoutMuscleData> {}
