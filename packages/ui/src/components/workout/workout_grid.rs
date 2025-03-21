use super::workout_grid_item::WorkoutGridItem;
use dioxus::prelude::*;
use fgdb::data::workout::WorkoutData;

#[component]
pub fn WorkoutGrid(
    workouts: Vec<WorkoutData>,
    on_workout_add: EventHandler<WorkoutData>,
) -> Element {
    rsx! {
        div {
            class: "h-full grid grid-cols-1 sm:grid-cols-2 gap-4 mt-2",
            for workout in workouts {
                WorkoutGridItem {
                    workout: workout,
                    on_add: on_workout_add
                }
            }
        }
    }
}
