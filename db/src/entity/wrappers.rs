use std::{borrow::Cow, collections::HashMap};

use sea_orm::DbErr;
use serde::{Deserialize, Serialize};
use validator::ValidationError;
use validator::ValidationErrors;

// https://doc.rust-lang.org/nomicon/hrtb.html
pub trait RequestableData: for<'de> Deserialize<'de> {}

#[derive(Debug, Deserialize)]
#[serde(bound = "T: RequestableData")]
pub struct RequestData<T>
where
    T: RequestableData,
{
    pub data: T,
    pub params: Option<HashMap<String, String>>,
}

// https://doc.rust-lang.org/nomicon/hrtb.html
pub trait ResponsableData: Serialize + for<'de> Deserialize<'de> {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "T: ResponsableData")]
pub struct ResponseData<T>
where
    T: ResponsableData,
{
    pub data: Option<T>,
    pub errors: Option<ValidationErrors>,
}

impl<T> ResponseData<T>
where
    T: ResponsableData,
{
    pub fn new(data: Option<T>, errors: Option<ValidationErrors>) -> ResponseData<T> {
        Self { data, errors }
    }
}

pub struct DbValidationErrors(DbErr);
impl From<DbErr> for DbValidationErrors {
    fn from(err: DbErr) -> Self {
        DbValidationErrors(err)
    }
}
impl From<DbValidationErrors> for ValidationErrors {
    fn from(wrapper: DbValidationErrors) -> Self {
        let mut errors = ValidationErrors::new();
        let field_error = match wrapper.0 {
            _ => ValidationError::new("internal").with_message(Cow::from("Internal server error")),
        };
        errors.add("data", field_error);
        errors
    }
}

// pub trait IntoValidationErrors {
//     fn to_validator_errors(self) -> ValidationErrors;
// }

// impl IntoValidationErrors for DbErr {
//     fn to_validator_errors(self) -> ValidationErrors {
//         // field is the param name, in this case generic data
//         // code is the rule name
//         let mut errors = ValidationErrors::new();
//         let field_error = match self {

//             _ => ValidationError::new("general").with_message(Cow::from("Internal server error")),
//         };
//         errors.add("data", field_error);
//         errors
//     }
// }
