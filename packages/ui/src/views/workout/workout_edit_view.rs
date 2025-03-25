use crate::{
    components::workout::{
        workout_edit_form::WorkoutEditForm,
        workout_muscles_for_workout_edit_form::WorkoutMusclesForWorkoutEditForm,
    },
    hooks::workout::use_workout_resource,
    views::Loading,
};
use dioxus::prelude::*;
use fgdb::data::{
    workout::{WorkoutInclude, WorkoutShowParams},
    HasIncludes,
};

#[component]
pub fn WorkoutEditView(workout_id: usize) -> Element {
    let params_sig = use_signal(|| {
        WorkoutShowParams {
            id: workout_id as i32,
            ..Default::default()
        }
        .with_include(WorkoutInclude::WorkoutMuscle(None))
    });
    // if workout_res returns None (workout does not exist), throwing will prop up to
    // error boundary
    let (workout_sig, workout_res) = use_workout_resource(params_sig);
    workout_res.suspend()?;

    rsx! {
        SuspenseBoundary {
            fallback: |_| rsx! { Loading {  }},
            match workout_sig() {
                Some(workout) => {
                    rsx! {
                        h1 { class: "text-2xl sm:text-3xl font-bold text-base-content", "Edit Workout" },
                        div {
                            class: "divider"
                        },
                        h2 { class: "text-xl sm:text-2xl font-bold text-base-content", "Edit Muscles" },

                        WorkoutMusclesForWorkoutEditForm{ workout: workout.clone()}
                        div {
                            class: "divider"
                        },
                        WorkoutEditForm{ workout}
                    }
                },
                None => {
                    rsx! { Loading {  } }
                }
            }
        }
    }
}
