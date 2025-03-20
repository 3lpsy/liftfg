#![allow(non_snake_case)]
use crate::components::workout::workout_grid::WorkoutGrid;
use crate::icons;
use crate::router;
use crate::services::get;
use crate::services::profile::get_profile;
use dioxus::prelude::*;
use fgdb::data::profile::ProfileData;
use fgdb::data::profile::ProfileShowParams;
use fgdb::data::workout::WorkoutData;
use fgdb::data::workout::WorkoutInclude;
use fgdb::data::workout::WorkoutIndexParams;
use fgdb::data::workout_muscle::WorkoutMuscleInclude;
use fgdb::data::HasIncludes;
use validator::ValidationErrors;

#[component]
pub fn WorkoutCreateView(profile_id: usize) -> Element {
    // TODO: error handling verbose, ideally handle errors higher up
    // and not in use_effect
    let mut profile_sig: Signal<Option<ProfileData>> = use_signal(|| None);
    let profile_res = use_resource(move || async move {
        get_profile(Some(ProfileShowParams {
            id: Some(profile_id as i32),
            name: None,
        }))
        .await
    })
    .suspend()?;
    use_effect(move || match profile_res() {
        Ok(profile) => {
            profile_sig.set(Some(profile.clone()));
        }
        Err(e) => {
            let mut app_errors = use_context::<Signal<ValidationErrors>>();
            app_errors.set(e.clone());
        }
    });

    // first we write get_workouts
    let mut workouts_ctx: Signal<Vec<WorkoutData>> = use_signal(|| vec![]);
    use_context_provider(|| workouts_ctx.clone());

    let workouts_res = use_resource(move || async move {
        get::<WorkoutIndexParams, Vec<WorkoutData>>(
            "workout_index",
            Some(
                WorkoutIndexParams::default().with_include(WorkoutInclude::WorkoutMuscle(Some(
                    vec![WorkoutMuscleInclude::Muscle],
                ))),
            ),
        )
        .await
    })
    .suspend()?;
    use_effect(move || match workouts_res() {
        Ok(data) => {
            *workouts_ctx.write() = data.clone();
        }
        Err(e) => {
            let mut app_errors = use_context::<Signal<ValidationErrors>>();
            app_errors.set(e.clone());
        }
    });

    // second, add a includes setup for profile to include workouts on response
    //
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
                    required: true,
                    placeholder: "Filter"
                }
            }
        }
        WorkoutGrid {}
    }
}
