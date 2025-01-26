//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "target_muscle")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::program_target_muscle::Entity")]
    ProgramTargetMuscle,
}

impl Related<super::program_target_muscle::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProgramTargetMuscle.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
