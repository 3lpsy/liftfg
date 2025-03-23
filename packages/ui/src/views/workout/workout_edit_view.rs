use dioxus::prelude::*;

#[component]
pub fn WorkoutEditView(workout_id: usize) -> Element {
    rsx! {
        h1 { class: "text-2xl sm:text-3xl font-bold text-base-content", "Edit Workout" },
        div {
            class: "divider"
        },
    }
}
