//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use anyhow::Result;
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
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "crate::entity::gym_profile::Entity")]
    GymProfile,
    #[sea_orm(has_many = "crate::entity::gym::Entity")]
    Gym,
    #[sea_orm(has_many = "crate::entity::program::Entity")]
    Program,
}

impl Related<crate::entity::gym_profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GymProfile.def()
    }
}

impl Related<crate::entity::program::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Program.def()
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

impl Entity {
    pub async fn by_name<C>(db: &C, name: &str) -> Result<Option<Model>, DbErr>
    where
        C: ConnectionTrait,
    {
        Self::find().filter(Column::Name.eq(name)).one(db).await
    }
    pub async fn by_default<C>(db: &C) -> Result<Option<Model>, DbErr>
    where
        C: ConnectionTrait,
    {
        Self::find()
            .filter(Column::IsDefault.eq(true))
            .one(db)
            .await
    }
}
