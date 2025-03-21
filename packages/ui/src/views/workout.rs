#![allow(non_snake_case)]
use crate::components::workout::workout_grid::WorkoutGrid;
use crate::filters::use_workouts_searched;
use crate::hooks::profile::use_profile_resource;
use crate::hooks::workout::use_workouts_resource;
use crate::icons;
use crate::router;

use dioxus::prelude::*;
use fgdb::data::profile::ProfileShowParams;

use fgdb::data::workout::WorkoutData;
use fgdb::data::workout::WorkoutInclude;
use fgdb::data::workout::WorkoutIndexParams;
use fgdb::data::workout_muscle::WorkoutMuscleInclude;
use fgdb::data::HasIncludes;

#[component]
pub fn WorkoutCreateView(profile_id: usize) -> Element {
    let profile_params_sig = use_signal(|| ProfileShowParams {
        id: Some(profile_id as i32),
        name: None,
    });
    let (_profile_sig, profile_res) = use_profile_resource(profile_params_sig);
    profile_res.suspend()?;

    let workouts_params_sig = use_signal(|| {
        WorkoutIndexParams::default().with_include(WorkoutInclude::WorkoutMuscle(Some(vec![
            WorkoutMuscleInclude::Muscle,
        ])))
    });
    let (workouts_sig, workouts_res) = use_workouts_resource(workouts_params_sig);
    workouts_res.suspend()?;

    let mut search_sig = use_signal(|| String::new());
    let workouts_searched_memo = use_workouts_searched(workouts_sig, search_sig);

    let handle_workout_add = move |workout: WorkoutData| {
        tracing::info!("Added workout: {}", workout.name);
    };

    rsx! {
        div {
            class: "flex justify-between items-center",
            h1 { class: "text-2xl sm:text-3xl font-bold text-base-content", "Add Workout" },
            Link {
                class: "btn btn-outline",
                to: router::Route::Home  {},
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
        p { "None" }
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
        WorkoutGrid { workouts: workouts_searched_memo(), on_workout_add: handle_workout_add}
    }
}
