use crate::{
    bindings::invoke,
    logging::{info, warn},
};
use fgdb::data::{
    profile::{ProfileGetParams, ProfileResponseData},
    ResponseData,
};
use fgutils::{
    constants::{VALIDATION_PARSING_CODE, VALIDATION_REQUEST_FIELD},
    verrors,
};
use gloo_timers::future::sleep;
use serde_wasm_bindgen::{from_value, to_value};
use std::time::Duration;
use validator::ValidationErrors;

pub async fn get_profile(
    args: Option<ProfileGetParams>,
) -> Result<Option<ProfileResponseData>, ValidationErrors> {
    info("Loading profile");
    sleep(Duration::from_secs(5)).await;
    info("Post wait");

    let args = to_value(&args.unwrap_or(ProfileGetParams::default()))
        .expect("Failed to convert ProfileGetParams to JsValue");
    match invoke("get_profile", args).await {
        Ok(result) => match from_value::<ResponseData<ProfileResponseData>>(result.clone()) {
            Ok(response) => Ok(response.data),
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
            let e = from_value::<ValidationErrors>(e.clone()).unwrap_or(verrors(
                VALIDATION_REQUEST_FIELD,
                VALIDATION_PARSING_CODE,
                format!("Error parsing request error: {:?}", e),
            ));
            Err(e)
        }
    }
}
