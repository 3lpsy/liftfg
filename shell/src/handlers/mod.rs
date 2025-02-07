use std::borrow::Cow;

use fgdb::entity::wrappers::RequestableData;
use tauri::{self, ipc::Invoke, ipc::InvokeBody};
use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};
pub mod user;

// error field will be request for generics (or parsed field)
// code will be parsing
pub fn parse_body<T>(body: InvokeBody) -> Result<T, ValidationErrors>
where
    T: RequestableData,
{
    match body {
        InvokeBody::Json(json) => serde_json::from_value(json).map_err(serde_to_validator_errors),
        InvokeBody::Raw(bytes) => {
            let s = String::from_utf8(bytes).map_err(|_e| {
                let mut errors = ValidationErrors::new();
                errors.add(
                    "request",
                    ValidationError::new("parsing")
                        .with_message(format!("Failed to convert raw bytes to UTF-8").into()),
                );
                errors
            })?;
            serde_json::from_str(&s).map_err(serde_to_validator_errors)
        }
    }
}
fn serde_to_validator_errors(e: serde_json::Error) -> ValidationErrors {
    let msg = e.to_string();

    let (field, message) = match msg {
        msg if msg.contains("missing field") => {
            let f = msg
                .split('`')
                .nth(1)
                .map(|s| s.to_string())
                .unwrap_or("request".to_string());
            (f.clone(), format!("missing field field: {}", f))
        }
        msg if msg.contains("unknown field") => {
            let f = msg
                .split('`')
                .nth(1)
                .map(|s| s.to_string())
                .unwrap_or("request".to_string());
            (f.clone(), format!("unknown field: {}", f))
        }
        _ => (
            "request".to_string(),
            "Unknown JSON parsing error".to_string(),
        ),
    };
    let mut errors = ValidationErrors::new();
    errors.errors_mut().insert(
        Cow::Owned(field),
        ValidationErrorsKind::Field(vec![
            ValidationError::new("parsing").with_message(Cow::Owned(message))
        ]),
    );
    errors
}

pub fn generate<R: tauri::Runtime>() -> impl Fn(Invoke<R>) -> bool + Send + Sync + 'static {
    tauri::generate_handler![user::create_user]
}
