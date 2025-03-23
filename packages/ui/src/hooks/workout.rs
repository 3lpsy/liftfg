use crate::services::get;
use dioxus::{
    hooks::{use_context, use_effect, use_resource},
    prelude::*,
    signals::Signal,
};
use fgdb::data::workout::{WorkoutData, WorkoutIndexParams, WorkoutShowParams};
use validator::ValidationErrors;

pub fn use_workout_resource(
    params_sig: Signal<WorkoutShowParams>,
) -> (
    Signal<Option<WorkoutData>>,
    Resource<Result<WorkoutData, ValidationErrors>>,
) {
    let mut data_sig: Signal<Option<WorkoutData>> = use_signal(|| None);
    let resource = use_resource(move || async move {
        get::<WorkoutShowParams, WorkoutData>("workout_show", Some(params_sig())).await
    });
    // Set up the effect to handle resource results
    use_effect(move || {
        if let Some(result) = resource() {
            match result {
                Ok(data) => {
                    *data_sig.write() = Some(data.clone());
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

pub fn use_workouts_resource(
    params_sig: Signal<WorkoutIndexParams>,
) -> (
    Signal<Vec<WorkoutData>>,
    Resource<Result<Vec<WorkoutData>, ValidationErrors>>,
) {
    let mut data_sig: Signal<Vec<WorkoutData>> = use_signal(|| vec![]);
    let resource = use_resource(move || async move {
        get::<WorkoutIndexParams, Vec<WorkoutData>>("workout_index", Some(params_sig())).await
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
