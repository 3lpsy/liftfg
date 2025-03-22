use dioxus::{
    hooks::{use_context, use_effect, use_resource},
    prelude::*,
    signals::Signal,
};
use fgdb::data::profile_workout::{ProfileWorkoutData, ProfileWorkoutIndexParams};
use validator::ValidationErrors;

use crate::services::get;

pub fn use_profile_workout_resource(
    params_sig: Signal<ProfileWorkoutIndexParams>,
) -> (
    Signal<Vec<ProfileWorkoutData>>,
    Resource<Result<Vec<ProfileWorkoutData>, ValidationErrors>>,
) {
    let mut data_sig: Signal<Vec<ProfileWorkoutData>> = use_signal(|| vec![]);
    let resource = use_resource(move || async move {
        get::<ProfileWorkoutIndexParams, Vec<ProfileWorkoutData>>(
            "profile_workout_index",
            Some(params_sig()),
        )
        .await
    });
    // Set up the effect to handle resource results
    use_effect(move || {
        if let Some(result) = resource() {
            match result {
                Ok(data) => {
                    *data_sig.write() = data.clone();
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
