use sea_orm::{prelude::DateTimeUtc, ActiveValue};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::entity::wrappers::{RequestableData, ResponsableData};

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct ProfileCreateData {
    #[validate(length(min = 1, max = 127))]
    pub name: String,
    pub is_default: Option<bool>,
}

impl From<ProfileCreateData> for super::entity::ActiveModel {
    fn from(profile_data: ProfileCreateData) -> Self {
        super::entity::ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(profile_data.name),
            is_default: ActiveValue::Set(profile_data.is_default.unwrap_or(false)),
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileResponseData {
    pub id: i32, // Using i32 since that's what's in the database
    pub name: String,
    pub is_default: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

impl From<super::entity::Model> for ProfileResponseData {
    fn from(model: super::entity::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            is_default: model.is_default,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl RequestableData for ProfileCreateData {}
impl ResponsableData for ProfileResponseData {}

#[cfg(test)]
mod tests {
    // use tracing::info;
    use validator::ValidationErrorsKind;

    use super::*;
    #[tokio::test]
    async fn it_validates_profile_data() {
        // fgcore::logging::init().unwrap();
        let create = ProfileCreateData {
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
