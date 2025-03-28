#![allow(non_snake_case)]
use crate::components::workout::workout_grid::WorkoutGrid;
use crate::filters::use_workouts_searched;
use crate::hooks::profile::use_profile_resource;
use crate::hooks::workout::use_workouts_resource;
use crate::icons;
use crate::logging;
use crate::router;

use crate::services::post;
use dioxus::prelude::*;
use fgdb::data::profile::ProfileShowParams;
use fgdb::data::profile_workout::ProfileWorkoutData;
use fgdb::data::profile_workout::ProfileWorkoutDeleteData;
use fgdb::data::profile_workout::ProfileWorkoutStoreData;
use fgdb::data::workout::WorkoutData;
use fgdb::data::workout::WorkoutInclude;
use fgdb::data::workout::WorkoutIndexParams;
use fgdb::data::workout_muscle::WorkoutMuscleInclude;
use fgdb::data::HasIncludes;
use validator::ValidationErrors;

#[component]
pub fn ProfileWorkoutCreateView(profile_id: usize) -> Element {
    let profile_id = profile_id as i32;
    let profile_params_sig = use_signal(|| ProfileShowParams {
        id: Some(profile_id),
        name: None,
    });
    let (_profile_sig, profile_res) = use_profile_resource(profile_params_sig);
    profile_res.suspend()?;

    let mut workouts_params_sig = use_signal(|| {
        WorkoutIndexParams::default()
            .with_include(WorkoutInclude::ProfileWorkout)
            .with_include(WorkoutInclude::WorkoutMuscle(vec![
                WorkoutMuscleInclude::Muscle,
            ]))
    });
    let (workouts_sig, workouts_res) = use_workouts_resource(workouts_params_sig);
    workouts_res.suspend()?;

    let unselected_workouts_memo = use_memo(move || {
        workouts_sig()
            .iter()
            .filter(|w| {
                w.profile_workout.as_ref().map_or(true, |pws| {
                    for pw in pws {
                        if pw.profile_id == profile_id {
                            return false;
                        }
                    }
                    return true;
                })
            })
            .cloned()
            .collect::<Vec<WorkoutData>>()
    });
    let mut search_sig = use_signal(|| String::new());
    let searched_workouts_memo = use_workouts_searched(unselected_workouts_memo, search_sig);

    let selected_workouts_memo = use_memo(move || {
        workouts_sig()
            .iter()
            .filter(|w| {
                w.profile_workout.as_ref().map_or(false, |pws| {
                    pws.iter().any(|pw| pw.profile_id == profile_id)
                })
            })
            .cloned()
            .collect::<Vec<WorkoutData>>()
    });

    let handle_workout_add = move |workout: WorkoutData| {
        // uniqueness needs to be handled
        spawn(async move {
            let data = ProfileWorkoutStoreData {
                profile_id,
                workout_id: workout.id,
            };
            match post::<ProfileWorkoutStoreData, ProfileWorkoutData>("profile_workout_store", data)
                .await
            {
                Ok(_) => {
                    tracing::info!("Added workout: {}", workout.name);
                    let params = workouts_params_sig.read().clone();
                    workouts_params_sig.set(params)
                }
                Err(e) => {
                    let mut app_errors = use_context::<Signal<ValidationErrors>>();
                    app_errors.set(e.clone());
                }
            }
        });
    };
    let handle_workout_delete = move |workout: WorkoutData| {
        spawn(async move {
            let data = ProfileWorkoutDeleteData {
                profile_id: Some(profile_id),
                workout_id: Some(workout.id),
                ..Default::default()
            };
            match post::<ProfileWorkoutDeleteData, ProfileWorkoutData>(
                "profile_workout_delete",
                data,
            )
            .await
            {
                Ok(_) => {
                    tracing::info!("Deleted workout: {}", workout.name);
                    let params = workouts_params_sig.read().clone();
                    workouts_params_sig.set(params)
                }
                Err(e) => {
                    let mut app_errors = use_context::<Signal<ValidationErrors>>();
                    app_errors.set(e.clone());
                }
            }
        });
    };

    rsx! {
        div {
            class: "flex justify-between items-center",
            h1 { class: "text-2xl sm:text-3xl font-bold text-base-content", "Add Workout" },
            Link {
                class: "btn btn-outline",
                to: router::Route::WorkoutCreateView  {},
                "Create Workout"
            }
        }
        div {
            class: "justify-center mt-2",
            p {
                "A workout guides your session and will determine what exercises you will be prompted for based on desired muscle groups and volume."
            }
        }

        div {
            class: "divider"
        }

        h2 { class: "text-xl sm:text-2xl font-bold text-base-content", "Selected Workouts" }
        div {
            class: "flex flex-row flex-wrap gap-2 my-2",
            if selected_workouts_memo().is_empty() {
                p {"None"}
            } else {
                for workout in selected_workouts_memo() {
                    button {
                        class: "btn btn-ghost btn-xs",
                        onclick: move |_evt| {
                          handle_workout_delete(workout.clone())
                        },
                        "{workout.name}",
                        span {
                            class:"text-sm",
                            "×"
                        }
                    }
                }
            }
        }

        div  {
            class: "mt-2",
            label {
                class: "input w-full",
                icons::SearchIcon{},
                input {
                    r#type:"search",
                    required: false,
                    placeholder: "Filter",
                    oninput: move |evt| {search_sig.set(evt.value());}
                }
            }
        }
        WorkoutGrid { workouts: searched_workouts_memo(), on_workout_add: handle_workout_add}
    }
}
