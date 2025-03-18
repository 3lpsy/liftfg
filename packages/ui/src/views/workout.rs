#![allow(non_snake_case)]
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
use fgdb::data::DefaultParams;
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
    let nav = navigator();
    use_effect(move || match profile_res() {
        Ok(profile) => {
            profile_sig.set(Some(profile.clone()));
        }
        Err(e) => {
            let mut app_errors = use_context::<Signal<ValidationErrors>>();
            app_errors.set(e.clone());
            nav.replace(router::Route::Errors {});
        }
    });

    // first we write get_workouts
    let mut workouts_ctx: Signal<Vec<WorkoutData>> = use_signal(|| vec![]);
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
    let nav = navigator();
    use_effect(move || match workouts_res() {
        Ok(data) => {
            *workouts_ctx.write() = data.clone();
        }
        Err(e) => {
            let mut app_errors = use_context::<Signal<ValidationErrors>>();
            app_errors.set(e.clone());
            nav.replace(router::Route::Errors {});
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

        // <div class="card w-96 bg-base-100 card-xs shadow-sm">
        //   <div class="card-body">
        //     <h2 class="card-title">Xsmall Card</h2>
        //     <p>A card component has a figure, a body part, and inside body there are title and actions parts</p>
        //     <div class="justify-end card-actions">
        //       <button class="btn btn-primary">Buy Now</button>
        //     </div>
        //   </div>
        // </div>
        div {
            class: "h-full grid grid-cols-1 sm:grid-cols-2 gap-4 mt-2",
            for workout in workouts_ctx() {
                div {
                    class: "card bg-base-100 card-xs shadow-sm",
                    div {
                        class: "card-body",
                        h2 { class: "card-title",  "{workout.name}" }
                        for workout_muscle in workout.workout_muscle.unwrap_or_default().iter() {
                            p {
                                strong {
                                "{workout_muscle.muscle.as_ref().unwrap().name}: "
                                }
                                "{workout_muscle.volume} sets"

                            }
                        }
                        div {
                            class: "card-actions justify-end",
                            button {
                                class: "btn",
                                "Add"
                            }
                        }
                    }
                }
            }
        }


    }
}
