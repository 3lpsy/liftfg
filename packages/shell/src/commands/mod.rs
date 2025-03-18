pub mod profile;
pub mod workout;
use std::borrow::Cow;

use fgdb::data::{
    DefaultDataType, DefaultParamsType, RequestData, RequestableData, RequestableParams,
};
use fgutils::constants::{
    VALIDATION_DATA_FIELD, VALIDATION_PARAMS_FIELD, VALIDATION_PARSING_CODE,
    VALIDATION_REQUEST_FIELD,
};
// use fgcore::controllers::profile::create_profile;
use tauri::{self, ipc::Invoke, ipc::InvokeBody};
use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

// error field will be request for generics (or parsed field)
// code will be parsing
//
pub fn parse_data<T>(body: InvokeBody) -> Result<T, ValidationErrors>
where
    T: RequestableData,
{
    // will throw if data is available but invalid
    let request: RequestData<T, DefaultParamsType> = parse_request(body)?;
    match request.data {
        Some(data) => Ok(data),
        None => Err(ValidationErrors::new()
            .with_error(
                VALIDATION_DATA_FIELD,
                ValidationError::new(VALIDATION_PARSING_CODE)
                    .with_message(format!("missing field field: data").into()),
            )
            .to_owned()),
    }
}

pub fn parse_params<T>(body: InvokeBody) -> Result<T, ValidationErrors>
where
    T: RequestableParams,
{
    // will throw if params is available but invalid
    let request: RequestData<DefaultDataType, T> = parse_request(body)?;
    match request.params {
        Some(params) => Ok(params),
        None => Err(ValidationErrors::new().with_error(
            VALIDATION_PARAMS_FIELD,
            ValidationError::new(VALIDATION_PARSING_CODE)
                .with_message(format!("missing field field: params").into()),
        )),
    }
}

fn parse_request<T, P>(body: InvokeBody) -> Result<RequestData<T, P>, ValidationErrors>
where
    T: RequestableData,
    P: RequestableParams,
{
    match body {
        InvokeBody::Json(json) => serde_json::from_value(json).map_err(serde_to_validator_errors),
        InvokeBody::Raw(bytes) => {
            let s = String::from_utf8(bytes).map_err(|_e| {
                let mut errors = ValidationErrors::new();
                errors.add(
                    VALIDATION_REQUEST_FIELD,
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
                .unwrap_or(VALIDATION_REQUEST_FIELD.to_string());
            (f.clone(), format!("missing field field: {}", f))
        }
        msg if msg.contains("unknown field") => {
            let f = msg
                .split('`')
                .nth(1)
                .map(|s| s.to_string())
                .unwrap_or(VALIDATION_REQUEST_FIELD.to_string());
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
            ValidationError::new(VALIDATION_PARSING_CODE).with_message(Cow::Owned(message))
        ]),
    );
    errors
}

// Routing names
pub fn generate<R: tauri::Runtime>() -> impl Fn(Invoke<R>) -> bool + Send + Sync + 'static {
    // It doesn't like reexports
    tauri::generate_handler![
        profile::profile_store::profile_store,
        profile::profile_show::profile_show,
        profile::profile_index::profile_index,
        profile::profile_update::profile_update,
        profile::profile_delete::profile_delete,
        workout::workout_index
    ]
}
