use dioxus::prelude::*;
use fgdb::data::{
    profile::ProfileData,
    profile_workout::{ProfileWorkoutInclude, ProfileWorkoutIndexParams},
    workout::WorkoutData,
    HasIncludes,
};

use crate::{hooks::profile_workout::use_profile_workout_resource, icons::ArrowRight, router};

#[component]
pub fn ProfileWorkoutIndexView() -> Element {
    let current_profile_ctx = use_context::<Signal<Option<ProfileData>>>();

    // asuming it's not None and None will be redirected in container
    let current_profile_id_memo =
        use_memo(move || current_profile_ctx().as_ref().map_or(0, |p| p.id));

    let profile_workout_params_sig = use_signal(|| {
        ProfileWorkoutIndexParams::default()
            .with_profile_id(current_profile_id_memo())
            .with_include(ProfileWorkoutInclude::Workout)
    });

    let (profile_workout_sig, profile_workout_res) =
        use_profile_workout_resource(profile_workout_params_sig);
    profile_workout_res.suspend()?;

    let workouts_memo: Memo<Vec<WorkoutData>> = use_memo(move || {
        profile_workout_sig()
            .iter()
            .filter(|pw| pw.workout.is_some()) // not possible to be none
            .map(|pw| pw.workout.as_ref().unwrap())
            .cloned()
            .collect()
    });
    rsx! {
        div {
            class: "flex justify-between items-center",
            h1 { class: "text-2xl sm:text-3xl font-bold text-base-content", "Selected Workouts" },
            Link {
                class: "btn btn-outline",
                to: router::Route::ProfileWorkoutCreateView  {profile_id: current_profile_id_memo() as usize},
                "Add/Remove Workouts"
            }
        }
        div {
            class: "divider"
        }
        ul { class: "list bg-base-100 rounded-box shadow-md",
            for workout in workouts_memo() {
                li {
                    class: "list-row",
                    div {
                        class: "flex-1 text-lg sm:text-xl font-semibold opacity-80 text-center uppercase self-center",
                            "{workout.name}"
                    }
                    div {}
                    Link {
                        class: "btn",
                        to: router::Route::WorkoutEditView { workout_id: workout.id as usize },
                        "Edit Workout"
                    }
                    a {
                        class: "btn",
                        "Start Session"
                    }
                }
            }
        }
    }
}
