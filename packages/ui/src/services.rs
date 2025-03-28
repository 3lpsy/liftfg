use crate::{bindings::invoke, logging};
use fgdb::data::{RequestableData, RequestableParams, ResponsableData, ResponseData};
use fgutils::{
    constants::{VALIDATION_PARSING_CODE, VALIDATION_REQUEST_FIELD},
    verrors,
};
use serde_wasm_bindgen::{from_value, to_value};
use validator::ValidationErrors;
use wasm_bindgen::JsValue;

// Generic Get
pub async fn get<T, R>(command: &str, args: Option<T>) -> Result<R, ValidationErrors>
where
    T: RequestableParams + Default,
    R: ResponsableData,
{
    logging::info!("Get: {}", command);
    let params = args.unwrap_or_default().as_params();
    let args = to_value(&params).expect("Failed to convert RequestParams to JsValue");
    call::<R>(command, args).await
}

// Generic Post
pub async fn post<T, R>(command: &str, args: T) -> Result<R, ValidationErrors>
where
    T: RequestableData,
    R: ResponsableData,
{
    logging::info!("Post: {}", command);
    let req = args.as_request();
    let args = to_value(&req).expect("Failed to convert RequestParams to JsValue");
    call::<R>(command, args).await
}

pub async fn call<R>(command: &str, args: JsValue) -> Result<R, ValidationErrors>
where
    R: ResponsableData,
{
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
                logging::info!("{:?}", &result);
                let e = verrors(
                    VALIDATION_REQUEST_FIELD,
                    VALIDATION_PARSING_CODE,
                    format!("Error parsing result: {:?}", e),
                );
                Err(e)
            }
        },
        Err(e) => {
            logging::error!("{:?}", &e);
            let re = from_value::<ResponseData<ValidationErrors>>(e.clone()).unwrap_or(
                ResponseData::from_errors(verrors(
                    VALIDATION_REQUEST_FIELD,
                    VALIDATION_PARSING_CODE,
                    format!("Error parsing request error: {:?}", e),
                )),
            );
            // theoretically, only errors should ever be populated
            Err(re.errors.unwrap())
        }
    }
}
