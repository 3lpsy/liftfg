use crate::entity::workout::{ExcerciseSplitStrategy, ExercisePromptStrategy, MuscleOrderStrategy};
#[cfg(feature = "db")]
use crate::entity::workout_muscle as entity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{ResponsableData, ResponseData};

// Responses
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkoutMuscleData {
    pub id: i32, // Using i32 since that's what's in the database
    pub name: String,
    pub code: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
// impls
#[cfg(feature = "db")]
impl From<entity::Model> for WorkoutData {
    fn from(model: entity::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            code: model.code,
            muscle_order_strategy: model.muscle_order_strategy,
            exercise_prompt_strategy: model.exercise_prompt_strategy,
            exercise_split_strategy: model.exercise_split_strategy,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<WorkoutData> for ResponseData<WorkoutData> {
    fn from(data: WorkoutData) -> Self {
        ResponseData {
            data: Some(data),
            errors: None,
            paginator: None,
        }
    }
}

impl ResponsableData for WorkoutData {}
impl ResponsableData for Vec<WorkoutData> {}
