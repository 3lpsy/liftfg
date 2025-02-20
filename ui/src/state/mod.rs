use crate::{
    bindings::invoke,
    logging::{info, warn},
};
use dioxus::signals::{GlobalSignal, Signal, Writable};
use fgdb::data::{
    profile::{ProfileGetParams, ProfileResponseData},
    ResponseData,
};
use fgutils::{
    constants::{VALIDATION_PARSING_CODE, VALIDATION_REQUEST_FIELD},
    verrors,
};
use serde_wasm_bindgen::{from_value, to_value};
use validator::ValidationErrors;

#[derive(Clone, Copy, Default, Debug)]
pub struct AppDataState {
    // current profile
    pub profile: Signal<Option<ProfileResponseData>>,
    // all profiles
    pub profiles: Signal<Vec<ProfileResponseData>>,
    pub state: Signal<State>,
}
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum State {
    #[default]
    Loading,
    Ready,
    Borked,
}

pub static APP_STATE: GlobalSignal<AppDataState> = Signal::global(|| AppDataState::default());
pub static APP_ERRORS: GlobalSignal<Vec<ValidationErrors>> = Signal::global(|| vec![]);

impl AppDataState {
    pub async fn load(args: Option<ProfileGetParams>) {
        info("Loading profile");
        let args = to_value(&args.unwrap_or(ProfileGetParams::default()))
            .expect("Failed to convert ProfileGetParams to JsValue");
        match invoke("get_profile", args).await {
            Ok(result) => match from_value::<ResponseData<ProfileResponseData>>(result.clone()) {
                Ok(response) => {
                    if let Some(profile) = response.data {
                        APP_STATE.write().profile.set(Some(profile))
                    }
                    // need way to determine that no profile exists but already loaded...
                    // how to we then redirect to welcome / profile creation
                    APP_STATE.write().state.set(State::Ready)
                }
                Err(e) => {
                    info(&format!("{:?}", &result));
                    APP_ERRORS.write().push(verrors(
                        VALIDATION_REQUEST_FIELD,
                        VALIDATION_PARSING_CODE,
                        format!("Error parsing result: {:?}", e),
                    ));
                    APP_STATE.write().state.set(State::Borked)
                }
            },
            Err(e) => {
                warn(&format!("Error: {:?}", &e));
                let e = from_value::<ValidationErrors>(e.clone()).unwrap_or(verrors(
                    VALIDATION_REQUEST_FIELD,
                    VALIDATION_PARSING_CODE,
                    format!("Error parsing request error: {:?}", e),
                ));
                APP_ERRORS.write().push(e);
                APP_STATE.write().state.set(State::Borked)
            }
        }
    }
}
