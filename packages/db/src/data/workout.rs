#[cfg(feature = "db")]
use crate::entity::workout as entity;
use chrono::{DateTime, Utc};
use fgutils::patterns::ALPHA_DASH;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    enums::{ExercisePromptStrategy, ExerciseSplitStrategy, MuscleOrderStrategy},
    profile::ProfileData,
    profile_workout::ProfileWorkoutData,
    workout_muscle::{WorkoutMuscleData, WorkoutMuscleInclude},
    HasIncludes, HasOrder, HasPagination, Includable, Order, Pagination, ResponsableData,
    ResponseData,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WorkoutInclude {
    ProfileWorkout,
    Profile,
    WorkoutMuscle(Option<Vec<WorkoutMuscleInclude>>),
}

impl Includable for WorkoutInclude {}

#[derive(Default, Clone, Debug, Validate, Serialize, Deserialize)]
pub struct WorkoutIndexParams {
    #[validate(range(min = 1, max = 256, message = "Profile ID must be between 1 and 256"))]
    pub profile_id: Option<i32>,
    #[validate(nested)]
    pub pagination: Option<Pagination>,
    #[validate(nested)]
    pub order: Option<Order>,
    #[validate(length(max = 3, message = "Max length of array is 3."))]
    pub includes: Option<Vec<WorkoutInclude>>,
}

#[derive(Debug, Validate, Serialize, Deserialize, Clone, PartialEq)]
pub struct WorkoutStoreData {
    #[validate(length(
        min = 1,
        max = 127,
        message = "Name must be between 1 and 127 characters long"
    ))]
    pub name: String,
    #[validate(length(
        min = 1,
        max = 127,
        message = "Code must be between 1 and 127 characters long"
    ), regex(
        path = *ALPHA_DASH,
        message="Field must only contain alphanumeric or -, ., _ characters")
    )]
    pub code: String,
    pub muscle_order_strategy: Option<MuscleOrderStrategy>,
    pub exercise_split_strategy: Option<ExerciseSplitStrategy>,
    pub exercise_prompt_strategy: Option<ExercisePromptStrategy>,
    #[validate(range(
        min = 1,
        max = 32,
        message = "Default exercise set split must be between 1 and 32"
    ))]
    pub exercise_set_split: Option<i32>,
}

impl Default for WorkoutStoreData {
    fn default() -> Self {
        WorkoutStoreData {
            name: "".to_string(),
            code: "".to_string(),
            muscle_order_strategy: Some(MuscleOrderStrategy::default()),
            exercise_split_strategy: Some(ExerciseSplitStrategy::default()),
            exercise_prompt_strategy: Some(ExercisePromptStrategy::default()),
            exercise_set_split: Some(3),
        }
    }
}

// Responses
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkoutData {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub muscle_order_strategy: MuscleOrderStrategy,
    pub exercise_prompt_strategy: ExercisePromptStrategy,
    pub exercise_split_strategy: ExerciseSplitStrategy,
    pub workout_muscle: Option<Vec<WorkoutMuscleData>>,
    pub profile_workout: Option<Vec<ProfileWorkoutData>>,
    pub profiles: Option<Vec<ProfileData>>,
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
            workout_muscle: None,
            profile_workout: None,
            profiles: None,
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

impl HasIncludes<WorkoutInclude> for WorkoutIndexParams {
    fn includes(&mut self) -> &mut Option<Vec<WorkoutInclude>> {
        &mut self.includes
    }
}

impl HasPagination for WorkoutIndexParams {
    fn pagination(&mut self) -> &mut Option<Pagination> {
        &mut self.pagination
    }
}

impl HasOrder for WorkoutIndexParams {
    fn order(&mut self) -> &mut Option<Order> {
        &mut self.order
    }
}
