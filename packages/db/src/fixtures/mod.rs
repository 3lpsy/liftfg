#[cfg(feature = "db")]
pub mod exercises;
pub use exercises::get_exercises_fixture;

// only db
pub const MUSCLE_DATA: &str = include_str!("./muscles.yaml");
pub const WORKOUT_DATA: &str = include_str!("./workouts.yaml");

use serde::{Deserialize, Serialize};

pub fn get_muscles_fixture() -> Vec<MuscleFixture> {
    let muscle_data: MusclesFixture =
        serde_yaml::from_str(MUSCLE_DATA).expect("Failed to parse YAML");
    muscle_data.muscles
}
pub fn get_workouts_fixture() -> Vec<WorkoutFixture> {
    let muscle_data: WorkoutsFixture =
        serde_yaml::from_str(WORKOUT_DATA).expect("Failed to parse YAML");
    muscle_data.workouts
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MuscleFixture {
    pub name: String,
    pub long_name: String,
    pub code: String,
    pub size_score: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MusclesFixture {
    pub muscles: Vec<MuscleFixture>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkoutMuscleFixture {
    pub code: String,
    pub volume: i64,
    pub priority: i64,
    pub exercise_set_split: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkoutFixture {
    pub name: String,
    pub code: String,
    pub muscles: Vec<WorkoutMuscleFixture>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkoutsFixture {
    pub workouts: Vec<WorkoutFixture>,
}
