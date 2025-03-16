#[cfg(feature = "db")]
//
use fgutils::constants::{VALIDATION_DATABASE_FIELD, VALIDATION_REQUEST_FIELD};
use sea_orm::{DbErr, TransactionError};
use std::borrow::Cow;
use validator::{ValidationError, ValidationErrors};

use crate::data::field_ref;

pub struct DbValidationErrors(DbErr);
impl From<DbErr> for DbValidationErrors {
    fn from(err: DbErr) -> Self {
        DbValidationErrors(err)
    }
}
impl From<TransactionError<DbErr>> for DbValidationErrors {
    fn from(err: TransactionError<DbErr>) -> Self {
        match err {
            TransactionError::Connection(e) => DbValidationErrors(e),
            TransactionError::Transaction(e) => DbValidationErrors(e),
        }
    }
}
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
