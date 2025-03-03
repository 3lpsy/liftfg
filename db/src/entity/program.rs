//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "program")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub profile_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::program_target_muscle::Entity")]
    ProgramTargetMuscle,
    #[sea_orm(
        belongs_to = "super::profile::Entity",
        from = "Column::ProfileId",
        to = "super::profile::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Profile,
}

impl Related<super::program_target_muscle::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProgramTargetMuscle.def()
    }
}

// Through pivot
impl Related<super::profile::Entity> for Entity {
    fn to() -> RelationDef {
        super::profile_program::Relation::Profile.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::profile_program::Relation::Profile.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
