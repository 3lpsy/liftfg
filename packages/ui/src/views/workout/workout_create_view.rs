use dioxus::prelude::*;

use crate::components::workout::workout_create_form::WorkoutCreateForm;

#[component]
pub fn WorkoutCreateView() -> Element {
    rsx! {
        h1 { class: "text-2xl sm:text-3xl font-bold text-base-content", "Create Workout" },
        div {
            class: "divider"
        },
        WorkoutCreateForm{}
    }
}
