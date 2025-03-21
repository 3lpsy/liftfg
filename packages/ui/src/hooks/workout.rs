use crate::services::get;
use dioxus::{
    hooks::{use_context, use_effect, use_resource},
    prelude::*,
    signals::Signal,
};
use fgdb::data::workout::{WorkoutData, WorkoutIndexParams};
use validator::ValidationErrors;

pub fn use_workouts_resource(
    params_sig: Signal<WorkoutIndexParams>,
) -> (
    Signal<Vec<WorkoutData>>,
    Resource<Result<Vec<WorkoutData>, ValidationErrors>>,
) {
    let mut workouts_sig: Signal<Vec<WorkoutData>> = use_signal(|| vec![]);
    let workouts_res = use_resource(move || async move {
        get::<WorkoutIndexParams, Vec<WorkoutData>>("workout_index", Some(params_sig())).await
    });
    // Set up the effect to handle resource results
    use_effect(move || {
        if let Some(result) = workouts_res() {
            match result {
                Ok(data) => {
                    *workouts_sig.write() = data.clone();
                }
                Err(e) => {
                    let mut app_errors = use_context::<Signal<ValidationErrors>>();
                    app_errors.set(e.clone());
                }
            }
        }
    });

    (workouts_sig, workouts_res)
}
