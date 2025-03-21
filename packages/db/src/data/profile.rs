#[cfg(feature = "db")]
use sea_orm::ActiveValue;

use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{RequestData, RequestableParams, ResponsableData, ResponseData};

#[cfg(feature = "db")]
use crate::entity::profile as entity;

// Responses
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProfileData {
    pub id: i32, // Using i32 since that's what's in the database
    pub name: String,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Requests
#[derive(Default, Clone, Debug, Validate, Serialize, Deserialize)]
pub struct ProfileShowParams {
    #[validate(range(min = 1))]
    pub id: Option<i32>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Validate, Serialize, Deserialize)]
pub struct ProfileDeleteParams {
    #[validate(range(min = 1, message = "Profile ID must be greater than 1"))]
    pub id: i32,
}

#[derive(Debug, Validate, Serialize, Deserialize, Default, Clone)]
pub struct ProfileStoreData {
    #[validate(length(
        min = 1,
        max = 127,
        message = "Name must be between 1 and 127 characters long"
    ))]
    pub name: String,
    pub is_default: Option<bool>,
}

#[derive(Debug, Validate, Serialize, Deserialize, Default, Clone)]
pub struct ProfileUpdateData {
    #[validate(range(min = 1, message = "Id must be at least greater than 1"))]
    pub id: i32,
    #[validate(length(
        min = 1,
        max = 127,
        message = "Name must be between 1 and 127 characters long"
    ))]
    pub name: Option<String>,
    pub is_default: Option<bool>,
}

// impls
impl From<ProfileData> for ProfileStoreData {
    fn from(data: ProfileData) -> Self {
        ProfileStoreData {
            name: data.name,
            is_default: Some(data.is_default),
        }
    }
}

impl From<ProfileData> for ProfileUpdateData {
    fn from(data: ProfileData) -> Self {
        ProfileUpdateData {
            id: data.id,
            name: Some(data.name),
            is_default: Some(data.is_default),
        }
    }
}

#[cfg(feature = "db")]
impl From<ProfileStoreData> for entity::ActiveModel {
    fn from(profile_data: ProfileStoreData) -> Self {
        entity::ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(profile_data.name),
            is_default: ActiveValue::Set(profile_data.is_default.unwrap_or(false)),
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        }
    }
}

impl<P: RequestableParams> From<ProfileStoreData> for RequestData<ProfileStoreData, P> {
    fn from(data: ProfileStoreData) -> Self {
        RequestData::from_data(data)
    }
}

#[cfg(feature = "db")]
impl From<entity::Model> for ProfileData {
    fn from(model: entity::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            is_default: model.is_default,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
impl From<ProfileData> for ResponseData<ProfileData> {
    fn from(data: ProfileData) -> Self {
        ResponseData {
            data: Some(data),
            errors: None,
            paginator: None,
        }
    }
}

impl ResponsableData for ProfileData {}
impl ResponsableData for Vec<ProfileData> {}

#[cfg(test)]
mod tests {
    // use tracing::info;
    use validator::ValidationErrorsKind;

    use super::*;
    #[tokio::test]
    async fn it_validates_profile_data() {
        // fgcore::logging::init().unwrap();
        let create = ProfileStoreData {
            name: "a".repeat(128),
            is_default: Some(false),
        };
        let r = create.validate();
        assert!(r.is_err());
        let errs = r.unwrap_err().into_errors();
        // info!("{:?}", &errs);
        assert!(errs.contains_key("name"));

        let ValidationErrorsKind::Field(name_errs) = errs.get("name").unwrap() else {
            unreachable!("We know this is a Field variant")
        };
        assert!(name_errs.len() == 1);
        assert!(&name_errs[0].code == "length");
    }
}
