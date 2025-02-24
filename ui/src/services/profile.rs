use crate::{
    bindings::invoke,
    logging::{info, warn},
};
use fgdb::data::{
    profile::{ProfileCreateData, ProfileData, ProfileGetParams},
    RequestableData, RequestableParams, ResponseData,
};
use fgutils::{
    constants::{VALIDATION_PARSING_CODE, VALIDATION_REQUEST_FIELD},
    verrors,
};
use gloo_timers::future::sleep;
use serde_wasm_bindgen::{from_value, to_value};
use std::time::Duration;
use validator::ValidationErrors;

// converted to params
pub async fn get_profile(args: Option<ProfileGetParams>) -> Result<ProfileData, ValidationErrors> {
    info("Loading profile");
    sleep(Duration::from_secs(2)).await;
    info("Post wait");
    let params = args.unwrap_or(ProfileGetParams::default()).to_params();
    let args = to_value(&params).expect("Failed to convert ProfileGetParams to JsValue");
    match invoke("get_profile", args).await {
        Ok(result) => match from_value::<ResponseData<ProfileData>>(result.clone()) {
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

// converted to request w/ data
pub async fn create_profile(args: ProfileCreateData) -> Result<ProfileData, ValidationErrors> {
    sleep(Duration::from_secs(1)).await;
    let req = args.as_request();
    let args = to_value(&req).expect("Failed to convert ProfileResponseData to JsValue");
    match invoke("create_profile", args).await {
        Ok(result) => match from_value::<ResponseData<ProfileData>>(result.clone()) {
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
