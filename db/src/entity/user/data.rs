use sea_orm::{prelude::DateTimeUtc, ActiveValue};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::entity::wrappers::{RequestableData, ResponsableData};

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct UserCreateData {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, max = 127))]
    pub name: String,
}

impl From<UserCreateData> for super::entity::ActiveModel {
    fn from(user_data: UserCreateData) -> Self {
        super::entity::ActiveModel {
            id: ActiveValue::NotSet,
            email: ActiveValue::Set(user_data.email),
            name: ActiveValue::Set(user_data.name),
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseData {
    pub id: i32, // Using i32 since that's what's in the database
    pub email: String,
    pub name: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

impl From<super::entity::Model> for UserResponseData {
    fn from(model: super::entity::Model) -> Self {
        Self {
            id: model.id,
            email: model.email,
            name: model.name,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl RequestableData for UserCreateData {}
impl ResponsableData for UserResponseData {}

#[cfg(test)]
mod tests {
    // use tracing::info;
    use validator::ValidationErrorsKind;

    use super::*;
    #[tokio::test]
    async fn it_validates_user_data() {
        // fgcore::logging::init().unwrap();
        let create = UserCreateData {
            name: "a".repeat(128),
            email: "notanemail".to_string(),
        };
        let r = create.validate();
        assert!(r.is_err());
        let errs = r.unwrap_err().into_errors();
        // info!("{:?}", &errs);
        assert!(errs.contains_key("name"));
        assert!(errs.contains_key("email"));

        let ValidationErrorsKind::Field(name_errs) = errs.get("name").unwrap() else {
            unreachable!("We know this is a Field variant")
        };
        let ValidationErrorsKind::Field(email_errs) = errs.get("email").unwrap() else {
            unreachable!("We know this is a Field variant")
        };
        assert!(name_errs.len() == 1);
        assert!(&name_errs[0].code == "length");
        assert!(email_errs.len() == 1);
        assert!(&email_errs[0].code == "email");
    }
}
