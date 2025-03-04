use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::entity::program as entity;

use super::{ResponsableData, ResponseData};

// Responses
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProgramData {
    pub id: i32, // Using i32 since that's what's in the database
    pub name: String,
    pub code: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
// impls

#[cfg(feature = "db")]
impl From<entity::Model> for ProgramData {
    fn from(model: entity::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            code: model.code,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<ProgramData> for ResponseData<ProgramData> {
    fn from(data: ProgramData) -> Self {
        ResponseData {
            data: Some(data),
            errors: None,
            paginator: None,
        }
    }
}

impl ResponsableData for ProgramData {}
impl ResponsableData for Vec<ProgramData> {}
