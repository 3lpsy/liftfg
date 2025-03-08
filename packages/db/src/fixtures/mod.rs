#[cfg(feature = "db")]
// only db
pub const INITIAL_MUSCLE_DATA: &str = include_str!("./initial_muscle_data.yaml");
pub const INITIAL_WORKOUT_DATA: &str = include_str!("./initial_workout_data.yaml");

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MuscleFixture {
    pub name: String,
    pub long_name: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MusclesFixture {
    pub muscles: Vec<MuscleFixture>,
}

pub fn get_muscle_data_fixture() -> Vec<MuscleFixture> {
    let muscle_data: MusclesFixture =
        serde_yaml::from_str(INITIAL_MUSCLE_DATA).expect("Failed to parse YAML");
    muscle_data.muscles
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkoutMuscleFixture {
    pub code: String,
    pub volume: u8,
    pub priority: u8,
    pub exercise_set_split: Option<u8>,
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

pub fn get_workout_data_fixture() -> Vec<WorkoutFixture> {
    let muscle_data: WorkoutsFixture =
        serde_yaml::from_str(INITIAL_WORKOUT_DATA).expect("Failed to parse YAML");
    muscle_data.workouts
}
