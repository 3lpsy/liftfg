use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{HasOrder, HasPagination, Order, Pagination, ResponsableData, ResponseData};

#[cfg(feature = "db")]
use crate::entity::muscle as entity;

#[derive(Default, Clone, Debug, Validate, Serialize, Deserialize)]
pub struct MuscleIndexParams {
    #[validate(nested)]
    pub pagination: Option<Pagination>,
    #[validate(nested)]
    pub order: Option<Order>,
}

// Responses
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MuscleData {
    pub id: i32, // Using i32 since that's what's in the database
    pub name: String,
    pub long_name: String,
    pub code: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(feature = "db")]
impl From<entity::Model> for MuscleData {
    fn from(model: entity::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            long_name: model.long_name,
            code: model.code,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
impl From<MuscleData> for ResponseData<MuscleData> {
    fn from(data: MuscleData) -> Self {
        ResponseData {
            data: Some(data),
            errors: None,
            paginator: None,
        }
    }
}

impl ResponsableData for MuscleData {}
impl ResponsableData for Vec<MuscleData> {}

impl HasPagination for MuscleIndexParams {
    fn pagination(&mut self) -> &mut Option<Pagination> {
        &mut self.pagination
    }
}

impl HasOrder for MuscleIndexParams {
    fn order(&mut self) -> &mut Option<Order> {
        &mut self.order
    }
}
