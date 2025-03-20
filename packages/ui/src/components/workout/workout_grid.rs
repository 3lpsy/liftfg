use super::workout_grid_item::WorkoutGridItem;
use dioxus::prelude::*;
use fgdb::data::workout::WorkoutData;

#[component]
pub fn WorkoutGrid() -> Element {
    let workouts_ctx = use_context::<Signal<Vec<WorkoutData>>>();
    rsx! {
        div {
            class: "h-full grid grid-cols-1 sm:grid-cols-2 gap-4 mt-2",
            for workout in workouts_ctx() {
                WorkoutGridItem{ workout: workout}
            }
        }
    }
}
