pub mod profile;

#[cfg(feature = "db")]
use fgutils::constants::VALIDATION_DATABASE_FIELD;
use fgutils::constants::VALIDATION_REQUEST_FIELD;
#[cfg(feature = "db")]
use sea_orm::DbErr;

use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap};
use validator::{ValidationError, ValidationErrors};

// validators requires &'static str
// TODO figure this out
static FIELD_NAMES: &[&str] = &["name", "is_default"];
pub fn field_ref(name: &str) -> &'static str {
    FIELD_NAMES
        .iter()
        .find(|&&x| x == name)
        .unwrap_or(&VALIDATION_REQUEST_FIELD)
}

// request data
// https://doc.rust-lang.org/nomicon/hrtb.html
pub trait RequestableData: for<'de> Deserialize<'de> + Serialize {
    fn to_request(self) -> RequestData<Self, DefaultParamsType> {
        RequestData {
            data: Some(self),
            params: None,
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct DefaultDataType {}
// seems kind of dangerous....
impl<T> RequestableData for T where T: for<'de> serde::Deserialize<'de> + serde::Serialize {}

// request params
pub type DefaultParamsType = HashMap<String, String>;
pub trait RequestableParams: for<'de> Deserialize<'de> + Serialize {
    fn to_params(self) -> RequestData<DefaultDataType, Self> {
        RequestData {
            data: None,
            params: Some(self),
        }
    }
}
impl<T> RequestableParams for T where T: for<'de> Deserialize<'de> + Serialize {}

// together
#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "T: RequestableData, P: RequestableParams")]
pub struct RequestData<T, P> {
    // body data
    pub data: Option<T>,
    // query-ish data
    pub params: Option<P>,
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

impl<T: ResponsableData> From<ValidationErrors> for ResponseData<T> {
    fn from(errors: ValidationErrors) -> Self {
        ResponseData {
            data: None,
            errors: Some(errors),
        }
    }
}
impl ResponsableData for ValidationErrors {}

#[cfg(feature = "db")]
pub struct DbValidationErrors(DbErr);
#[cfg(feature = "db")]
impl From<DbErr> for DbValidationErrors {
    fn from(err: DbErr) -> Self {
        DbValidationErrors(err)
    }
}
#[cfg(feature = "db")]
impl From<DbValidationErrors> for ValidationErrors {
    // field request (generic where it happened)
    // code database (specific failure about what rule was broken, in this case a generic database rule)
    fn from(wrapper: DbValidationErrors) -> Self {
        // TODO messages in realase
        let db_msg = wrapper.0.to_string();
        let default_msg = db_msg.clone();

        match db_msg {
            msg if msg.contains("UNIQUE constraint failed:") => {
                let field = msg
                    .split("UNIQUE constraint failed:")
                    .nth(1)
                    .and_then(|s| s.trim().split('.').nth(1))
                    .map(|s| s.to_string()) // Convert to owned String
                    .unwrap_or_else(|| VALIDATION_REQUEST_FIELD.to_string());

                ValidationErrors::new().with_error(
                    field_ref(&field),
                    ValidationError::new("unique")
                        .with_message(Cow::from(format!("Field {field} must be unique"))),
                )
            }
            _ => ValidationErrors::new().with_error(
                VALIDATION_REQUEST_FIELD,
                ValidationError::new(VALIDATION_DATABASE_FIELD)
                    .with_message(Cow::from(default_msg)),
            ),
        }
    }
}
