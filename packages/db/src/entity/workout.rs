//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(1))")]
pub enum MuscleOrderStrategy {
    #[sea_orm(string_value = "DETERMINISTIC")]
    Deterministic,
    // Put the last hit muscle first
    #[sea_orm(string_value = "LEAST_WORKED")]
    LeastWorked,
    // Rotating, rotates by priority given the first muscle of the last session
    #[sea_orm(string_value = "ROTATING")]
    Rotating,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(1))")]
pub enum ExcerciseSplitStrategy {
    // splits 4 into 2+2
    // splits 5 into 3+2
    // splits 6 into 3+3
    // splits 7 into 3+2+2
    // splits 8 into 3+3+2
    // splits 9 into 3+3+3
    // basically minimum 2 sets for exercise, prefer 3
    #[sea_orm(string_value = "SIMPLE")]
    Simple,
    // on x, always prefer that split no matter remainder
    #[sea_orm(string_value = "NO_ADJUST")]
    NoAdjust,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(1))")]
pub enum ExercisePromptStrategy {
    #[sea_orm(string_value = "COMMON_COMPOUND")]
    CommonCompound, // default, prompt for most common first, prefer compounds
    #[sea_orm(string_value = "COMMON_ISOLATION")]
    CommonIsolation, // default, prompt for most common first, prefer isolation for warmup
    #[sea_orm(string_value = "COMMON_COMPOUND_ROTATE")]
    CommonCompoundRotate, // prompt for the most common but rotate
    #[sea_orm(string_value = "COMMON_PREVIOUS")]
    CommonPrevious, // prefer previously done, prompt for common
    #[sea_orm(string_value = "CHAOTIC")]
    Chaotic, // truly random
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "workout")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    #[sea_orm(unique)]
    pub code: String,
    pub muscle_order_strategy: MuscleOrderStrategy,
    pub exercise_split_strategy: ExcerciseSplitStrategy,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::profile::Entity")]
    Profile,
}

// Through pivot
impl Related<super::profile::Entity> for Entity {
    fn to() -> RelationDef {
        super::profile_workout::Relation::Profile.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::profile_workout::Relation::Profile.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
