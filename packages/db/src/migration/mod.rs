#[cfg(feature = "db")]
// only db
pub use sea_orm_migration::prelude::*;
pub mod common;
pub mod m20220101_000001_create_profile;
pub mod m20250114_010338_create_program;
pub mod m20250115_171424_create_gym;
pub mod m20250115_173443_create_target_muscle;
pub mod m20250115_175632_create_exercise;
pub mod m20250115_192036_create_workout;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_profile::Migration),
            Box::new(m20250114_010338_create_program::Migration),
            Box::new(m20250115_171424_create_gym::Migration),
            Box::new(m20250115_173443_create_target_muscle::Migration),
            Box::new(m20250115_175632_create_exercise::Migration),
            Box::new(m20250115_192036_create_workout::Migration),
        ]
    }
}
