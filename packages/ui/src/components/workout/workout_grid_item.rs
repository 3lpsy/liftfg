use dioxus::prelude::*;
use fgdb::data::workout::WorkoutData;

#[component]
pub fn WorkoutGridItem(workout: WorkoutData) -> Element {
    rsx! {
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
