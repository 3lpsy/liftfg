#[cfg(feature = "db")]
// only db
use serde::{Deserialize, Serialize};
pub const ABS_DATA: &str = include_str!("./abs.yaml");
pub const BICEPS_DATA: &str = include_str!("./biceps.yaml");
pub const CHEST_DATA: &str = include_str!("./chest.yaml");
pub const FRONT_DELTS_DATA: &str = include_str!("./front_delts.yaml");
pub const GLUTES_DATA: &str = include_str!("./glutes.yaml");
pub const HAMSTRINGS_DATA: &str = include_str!("./hamstrings.yaml");
pub const LATS_DATA: &str = include_str!("./lats.yaml");
pub const LOWER_BACK_DATA: &str = include_str!("./lower_back.yaml");
pub const QUADS_DATA: &str = include_str!("./quads.yaml");
pub const REAR_DELTS_DATA: &str = include_str!("./rear_delts.yaml");
pub const SIDE_DELTS_DATA: &str = include_str!("./side_delts.yaml");
pub const THIGHS_DATA: &str = include_str!("./thighs.yaml");
pub const TRICEPS_DATA: &str = include_str!("./triceps.yaml");

pub fn get_exercises_fixture() -> Vec<ExerciseFixture> {
    [
        ABS_DATA,
        BICEPS_DATA,
        CHEST_DATA,
        FRONT_DELTS_DATA,
        GLUTES_DATA,
        HAMSTRINGS_DATA,
        LATS_DATA,
        LOWER_BACK_DATA,
        QUADS_DATA,
        REAR_DELTS_DATA,
        SIDE_DELTS_DATA,
        THIGHS_DATA,
        TRICEPS_DATA,
    ]
    .iter()
    .flat_map(|&data| {
        let fixture: ExercisesFixture = serde_yaml::from_str(data).expect("Failed to parse YAML");
        fixture.exercises
    })
    .collect()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExerciseMuscleFixture {
    pub code: String,
    pub effectiveness: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExerciseFixture {
    pub name: String,
    pub code: String,
    pub movement_code: String,
    pub equipment_type: String,
    pub fatigue_score: u8,
    pub muscles: Vec<ExerciseMuscleFixture>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExercisesFixture {
    pub exercises: Vec<ExerciseFixture>,
}
