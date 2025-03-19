//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use anyhow::Result;
use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "profile")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub name: String,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // direct
    #[sea_orm(has_many = "super::gym_profile::Entity")]
    GymProfile,
    #[sea_orm(has_many = "super::profile_workout::Entity")]
    ProfileWorkout,
    // indirect
    #[sea_orm(has_many = "super::gym::Entity")]
    Gym,
    #[sea_orm(has_many = "super::workout::Entity")]
    Workout,
}
// direct
impl Related<super::gym_profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GymProfile.def()
    }
}

impl Related<super::profile_workout::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProfileWorkout.def()
    }
}

//indirect
impl Related<crate::entity::workout::Entity> for Entity {
    // The final relation is Cake -> CakeFilling -> Filling
    fn to() -> RelationDef {
        crate::entity::profile_workout::Relation::Workout.def()
    }

    fn via() -> Option<RelationDef> {
        // The original relation is CakeFilling -> Cake,
        // after `rev` it becomes Cake -> CakeFilling
        Some(
            crate::entity::profile_workout::Relation::Profile
                .def()
                .rev(),
        )
    }
}

impl Related<crate::entity::gym::Entity> for Entity {
    // The final relation is Cake -> CakeFilling -> Filling
    fn to() -> RelationDef {
        crate::entity::gym_profile::Relation::Gym.def()
    }

    fn via() -> Option<RelationDef> {
        // The original relation is CakeFilling -> Cake,
        // after `rev` it becomes Cake -> CakeFilling
        Some(crate::entity::gym_profile::Relation::Profile.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

// Query Helpers
//
impl Entity {
    pub async fn by_name<C>(dbc: &C, name: &str) -> Result<Option<Model>, DbErr>
    where
        C: ConnectionTrait,
    {
        Self::find().filter(Column::Name.eq(name)).one(dbc).await
    }
    pub async fn by_default<C>(dbc: &C) -> Result<Option<Model>, DbErr>
    where
        C: ConnectionTrait,
    {
        Self::find()
            .filter(Column::IsDefault.eq(true))
            .one(dbc)
            .await
    }
}
