//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "set")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub reps: i32,
    pub weight: i32,
    pub workout_id: i32,
    pub exercise_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::exercise::Entity",
        from = "Column::ExerciseId",
        to = "super::exercise::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Exercise,
    #[sea_orm(
        belongs_to = "super::workout::Entity",
        from = "Column::WorkoutId",
        to = "super::workout::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Workout,
}

impl Related<super::exercise::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Exercise.def()
    }
}

impl Related<super::workout::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Workout.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
