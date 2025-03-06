#[cfg(feature = "db")]
// only db
pub const INITIAL_MUSCLE_DATA: &str = include_str!("./initial_muscle_data.yaml");
pub const INITIAL_PROGRAM_DATA: &str = include_str!("./initial_program_data.yaml");

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
pub struct ProgramMuscleFixture {
    pub code: String,
    pub volume: u8,
    pub order: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgramFixture {
    pub name: String,
    pub code: String,
    pub muscles: Vec<ProgramMuscleFixture>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgramsFixture {
    pub programs: Vec<ProgramFixture>,
}

pub fn get_program_data_fixutre() -> Vec<ProgramFixture> {
    let muscle_data: ProgramsFixture =
        serde_yaml::from_str(INITIAL_MUSCLE_DATA).expect("Failed to parse YAML");
    muscle_data.programs
}
