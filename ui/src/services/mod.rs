pub mod profile;

use crate::{
    bindings::invoke,
    logging::{info, warn},
};
use fgdb::data::{RequestableData, RequestableParams, ResponsableData, ResponseData};
use fgutils::{
    constants::{VALIDATION_PARSING_CODE, VALIDATION_REQUEST_FIELD},
    verrors,
};
use serde_wasm_bindgen::{from_value, to_value};
use validator::ValidationErrors;

// Generic Get
pub async fn get<T, R>(command: &str, args: Option<T>) -> Result<R, ValidationErrors>
where
    T: RequestableParams + Default,
    R: ResponsableData,
{
    let params = args.unwrap_or_default().to_params();
    let args = to_value(&params).expect("Failed to convert RequestParams to JsValue");
    match invoke(command, args).await {
        Ok(result) => match from_value::<ResponseData<R>>(result.clone()) {
            Ok(response) => match (response.data, response.errors) {
                (Some(data), _) => Ok(data),
                (_, Some(errors)) => Err(errors),
                (None, None) => Err(verrors(
                    VALIDATION_REQUEST_FIELD,
                    VALIDATION_PARSING_CODE,
                    "Error parsing result: No data or errors".to_string(),
                )),
            },
            Err(e) => {
                info(&format!("{:?}", &result));
                let e = verrors(
                    VALIDATION_REQUEST_FIELD,
                    VALIDATION_PARSING_CODE,
                    format!("Error parsing result: {:?}", e),
                );
                Err(e)
            }
        },
        Err(e) => {
            warn(&format!("Error: {:?}", &e));
            let re =
                from_value::<ResponseData<ValidationErrors>>(e.clone()).unwrap_or(ResponseData {
                    data: None,
                    errors: Some(verrors(
                        VALIDATION_REQUEST_FIELD,
                        VALIDATION_PARSING_CODE,
                        format!("Error parsing request error: {:?}", e),
                    )),
                });
            // theoretically, only errors should ever be populated
            Err(re.errors.unwrap())
        }
    }
}

// Generic Post
pub async fn post<T, R>(command: &str, args: T) -> Result<R, ValidationErrors>
where
    T: RequestableData,
    R: ResponsableData,
{
    let req = args.as_request();
    let args = to_value(&req).expect("Failed to convert RequestParams to JsValue");
    match invoke(command, args).await {
        Ok(result) => match from_value::<ResponseData<R>>(result.clone()) {
            Ok(response) => match (response.data, response.errors) {
                (Some(data), _) => Ok(data),
                (_, Some(errors)) => Err(errors),
                (None, None) => Err(verrors(
                    VALIDATION_REQUEST_FIELD,
                    VALIDATION_PARSING_CODE,
                    "Error parsing result: No data or errors".to_string(),
                )),
            },
            Err(e) => {
                info(&format!("{:?}", &result));
                let e = verrors(
                    VALIDATION_REQUEST_FIELD,
                    VALIDATION_PARSING_CODE,
                    format!("Error parsing result: {:?}", e),
                );
                Err(e)
            }
        },
        Err(e) => {
            let re =
                from_value::<ResponseData<ValidationErrors>>(e.clone()).unwrap_or(ResponseData {
                    data: None,
                    errors: Some(verrors(
                        VALIDATION_REQUEST_FIELD,
                        VALIDATION_PARSING_CODE,
                        format!("Error parsing request error: {:?}", e),
                    )),
                });
            // theoretically, only errors should ever be populated
            Err(re.errors.unwrap())
        }
    }
}
