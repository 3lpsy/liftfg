use dioxus::{
    hooks::{use_context, use_effect, use_resource},
    prelude::*,
    signals::Signal,
};
use fgdb::data::profile::{ProfileData, ProfileShowParams};
use validator::ValidationErrors;

use crate::services::get;

pub fn use_profile_resource(
    params_sig: Signal<ProfileShowParams>,
) -> (
    Signal<Option<ProfileData>>,
    Resource<Result<ProfileData, ValidationErrors>>,
) {
    let mut data_sig: Signal<Option<ProfileData>> = use_signal(|| None);

    let resource = use_resource(move || async move {
        get::<ProfileShowParams, ProfileData>("profile_show", Some(params_sig())).await
    });

    use_effect(move || {
        if let Some(result) = resource() {
            match result {
                Ok(profile) => {
                    data_sig.set(Some(profile.clone()));
                }
                Err(e) => {
                    let mut app_errors = use_context::<Signal<ValidationErrors>>();
                    app_errors.set(e.clone());
                }
            }
        }
    });

    (data_sig, resource)
}
